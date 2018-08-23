#![feature(box_patterns)]

extern crate rkdb;

use rkdb::{
    api,
    kbindings::*
};

// Attempts to connect to localhost port 12001, runs a query and prints result
fn main() {
    let handle = match api::Handle::connect("localhost", 12001, "user") {
        Ok(h) => h,
        Err(e) => { println!("{}", e); std::process::exit(1) }
    };
    let query = "([]a:til 10;b:reverse til 10;c:10?`4;d:{x#.Q.a}each til 10)";
    let k = match handle.query(query) {
        Ok(h) => h,
        Err(e) => { println!("{}", e); std::process::exit(1) }
    };
    handle.close();
    let KOwned(k) = k;
    println!("{:?}", KVal::new(k));
}
