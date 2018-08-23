extern crate rkdb;
extern crate num;

use std::{thread, time};
use rkdb::{
    types::K,
    k::*,
    kbindings::*
};

#[no_mangle]
pub extern fn kprinter(k: *const K) -> *const K {
    println!("{:?}", KVal::new(k));
    kvoid()
}

#[no_mangle]
pub extern fn ksleep(k: *const K) -> *const K {
    match KVal::new(k) {
        KVal::Int(KData::Atom(&mut i)) => sleep(i as u32),
        KVal::Long(KData::Atom(&mut i)) =>  sleep(i as u32),
        _ => return kerror("Please supply an int")
    }
    kvoid()
}

fn sleep(i: u32) {
    if i < 1 {
        println!("Value must be greater than 0")
    } else {
        thread::sleep(time::Duration::from_millis(i as u64))
    }
}

#[no_mangle]
pub extern fn kmean(k: *const K) -> *const K {
    match KVal::new(k) {
        KVal::Long(KData::List(l)) => mean(l),
        KVal::Int(KData::List(l)) => mean(l),
        KVal::Float(KData::List(l)) => mean(l),
        KVal::Real(KData::List(l)) => mean(l),
        _ => kerror("nyi")
    }
}

fn mean<N: num::Num + num::ToPrimitive + Copy>(nums: &[N]) -> *const K {
    let sum = nums.iter().fold(N::zero(), |acc, &v| acc + v);
    unsafe { kf(sum.to_f64().unwrap() / nums.len() as f64) }
}

#[no_mangle]
pub extern fn ksum(k: *const K) -> *const K {
    match KVal::new(k) {
        KVal::Long(KData::List(l)) => sum(l),
        KVal::Int(KData::List(l)) => sum(l),
        KVal::Float(KData::List(l)) => sum(l),
        KVal::Real(KData::List(l)) => sum(l),
        _ => kerror("nyi")
    }
}

fn sum<N: num::Num + num::ToPrimitive + Copy>(nums: &[N]) -> *const K {
    let sum = nums.iter().fold(N::zero(), |acc, &v| acc + v);
    unsafe { kf(sum.to_f64().unwrap()) }
}

#[no_mangle]
pub extern fn kdictex(_: *const K) -> *const K {
    let mut a1 = intern_strings(vec!("Once", "Twice", "Thrice").iter().map(|s| s.to_string()).collect());
    let mut a2 = vec!(1, 2, 3);
    let v1 = KVal::Symbol(KData::List(&mut a1));
    let v2 = KVal::Int(KData::List(&mut a2));
    kdict(&v1, &v2)
}

#[no_mangle]
pub extern fn ktableex(_: *const K) -> *const K {
    let mut s1 = intern_strings(vec!("Once", "Twice", "Thrice").iter().map(|s| s.to_string()).collect());
    let mut i1 = vec!(1, 2, 3);
    let mut i2 = vec!(2, 3, 4);
    let mut i3 = vec!(3.0, 4.1, 5.5);
    let k = KVal::Symbol(KData::List(&mut s1));
    let v1 = KVal::Int(KData::List(&mut i1));
    let v2 = KVal::Long(KData::List(&mut i2));
    let v3 = KVal::Float(KData::List(&mut i3));
    let v = KVal::Mixed(vec!(v1, v2, v3));
    let d = KVal::Dict(Box::new(k), Box::new(v));
    ktable(d)
}
