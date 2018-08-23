extern crate syn;

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub const KDB_SYM_FILE: &'static str = "symbols.rs";

fn parse_fns(fns: &syn::ItemForeignMod) -> Vec<String> {
    let mut functions = vec![];
    for item in &fns.items {
        match item {
            syn::ForeignItem::Fn(ref item_fn) => {
                let mut sym: String = "_".to_owned();
                let ident = item_fn.ident.to_string();
                sym.push_str(&ident);

                functions.push(sym);
            }
            _ => (),
        }
    }

    functions
}

fn ksyms(fname: &str) -> Vec<String> {
    let mut src = String::new();
    let mut file = File::open(&fname).expect("Unable to open file");
    file.read_to_string(&mut src).expect("Unable to read file");

    let syntax = syn::parse_file(&src).expect("Unable to parse file");

    syntax.items.iter().map(|item| match item {
            syn::Item::ForeignMod(ref item_fn) => parse_fns(item_fn),
            _ => vec![],
        })
        .flat_map(|x| x)
        .collect()
}

fn sym_file() -> std::path::PathBuf {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable not provided");
    
    Path::new(&out_dir).join(KDB_SYM_FILE)
}

fn main() {
    let base = env::current_dir().unwrap();

    let src = base.join("src").join("k.rs");
    
    let syms = ksyms(src.to_str().unwrap());

    let dest_path = sym_file();

    let mut f = File::create(&dest_path)
        .expect("Could not create file to store external KDB symbols");
    
    writeln!(f, "#[allow(dead_code)]")
        .expect("Could not write to KDB symbol file header");
    writeln!(f, "pub const SYMBOLS: [&'static str; {}] = {:?};", syms.len(), syms)
        .expect("Could not write to KDB symbol file");   

    f.flush()
        .expect("Could not flush write to KDB symbol");

    let dir = base.join("src").join("c");
    println!("cargo:rustc-link-search={}", dir.to_str().unwrap());
}
