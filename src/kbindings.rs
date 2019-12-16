use std;
use std::mem::{self, zeroed};
use std::slice;
use std::ptr;
use std::fmt;
use std::ffi;
use std::collections::HashMap;
use std::sync::Mutex;

use types::*;

#[cfg(not(feature="api"))]
use k::*;

#[cfg(feature="api")]
use kapi::*;

lazy_static! {
    static ref ERR_STRS: Mutex<HashMap<String, ffi::CString>> = {
        let m = HashMap::with_capacity(64);
        Mutex::new(m)
    };
}


#[derive(Debug)]
pub struct KOwned(pub &'static K);

impl Drop for KOwned {
    fn drop(&mut self) {
        unsafe { r0(self.0) };
    }
}

impl KOwned {
    pub fn new(bytes: &[u8]) -> KOwned {
        unsafe {
            // make a new, empty K struct holding a bytelist
            let k = ktn(4, bytes.len() as i64);
            let sx = (*k).fetch_slice::<u8>();
            assert_eq!(bytes.len(), sx.len());
            sx.clone_from_slice(bytes);
            KOwned(&*k)
        }
    }
}


// these are accessors for the (untagged) union
impl K {
    #[inline]
    pub unsafe fn cast<'a, T: fmt::Debug>(&self) -> &'a mut T {
        let u = &self.union as *const u8;
        &mut *(u as *mut T)
    }
    
    pub unsafe fn cast_with_ptr_offset<'a, T>(&self) -> &'a mut T {
        let u = &self.union as *const u8;
        &mut *(u.add(mem::size_of::<usize>()) as *mut T)
    }

    #[inline]
    pub unsafe fn fetch_slice<'a, T:'a>(&self) -> &'a mut [T] {
        slice::from_raw_parts_mut(self.cast_with_ptr_offset(), *self.cast())
    }
}

impl ::std::default::Default for K {
    fn default() -> Self { unsafe { zeroed() } }
}

#[derive(Debug)]
pub enum KData<'a, T: 'a> {
    Atom(&'a mut T),
    List(&'a mut [T])
}

impl<'a, T: 'a + fmt::Debug> KData<'a, T>{
    #[inline]
    unsafe fn atom(k: &'a K) -> KData<'a, T> {
        KData::Atom(k.cast())
    }

    #[inline]
    unsafe fn guid_atom(k: &'a K) -> KData<'a, T> {
        KData::Atom(k.cast_with_ptr_offset()) // while this is an atom, it is packed into a list of 1
    }

    #[inline]
    unsafe fn list(k: &'a K) -> KData<'a, T> {
        KData::List(k.fetch_slice())
    }
}


#[derive(Debug)]
pub enum KVal<'a> {
    Mixed(Vec<KVal<'a>>),
    Bool(KData<'a, bool>),
    Guid(KData<'a, [u8; 16]>),
    Byte(KData<'a, u8>),
    Short(KData<'a, i16>),
    Int(KData<'a, i32>),
    Long(KData<'a, i64>),
    Real(KData<'a, f32>),
    Float(KData<'a, f64>),
    Char(&'a i8),
    String(&'a str),
    Err(&'a str),
    Symbol(KData<'a, String>),
    Table(Box<KVal<'a>>),
    Dict(Box<KVal<'a>>, Box<KVal<'a>>), // Keys, Values
    Timestamp(KData<'a, i64>),
    Month(KData<'a, i32>),
    Date(KData<'a, i32>),
    Datetime(KData<'a, f64>),
    Timespan(KData<'a, i64>),
    Minute(KData<'a, i32>),
    Second(KData<'a, i32>),
    Time(KData<'a, i32>),
    Function,
    Unknown,
}

impl<'a> KVal<'a> {
    pub unsafe fn from_raw(k: *const K) -> KVal<'a> {
        Self::new(&*k)
    }

    pub fn new(k: &'a K) -> KVal<'a> {
        unsafe {
            match k.t {
                -1 => KVal::Bool(KData::atom(k)),
                -2 => KVal::Guid(KData::guid_atom(k)), 
                -4 => KVal::Byte(KData::atom(k)),
                -5 => KVal::Short(KData::atom(k)),
                -6 => KVal::Int(KData::atom(k)),
                -7 => KVal::Long(KData::atom(k)),
                -8 => KVal::Real(KData::atom(k)),
                -9 => KVal::Float(KData::atom(k)),
                -10 => KVal::Char(k.cast()),
                -11 => KVal::Symbol(KData::atom(k)),
                -12 => KVal::Timestamp( KData::atom(k)),
                -13 => KVal::Month( KData::atom(k)),
                -14 => KVal::Date( KData::atom(k)),
                -15 => KVal::Datetime( KData::atom(k)),
                -16 => KVal::Timespan( KData::atom(k)),
                -17 => KVal::Minute( KData::atom(k)),
                -18 => KVal::Second( KData::atom(k)),
                -19 => KVal::Time( KData::atom(k)),
                -128 => {
                    let err = {
                        let mut ptr: [u8; 8] = [0; 8];
                        ptr.clone_from_slice(&(*k).union[0..8]);
                        ffi::CStr::from_ptr(std::mem::transmute::<[u8; 8], *const i8>(ptr))
                    };
                    KVal::Err(err.to_str().unwrap())
                }
                0 => {
                    let s: &[&K] = k.fetch_slice();
                    KVal::Mixed(s.iter().map(|&x| KVal::new(x)).collect())
                },
                1  => KVal::Bool(   KData::list(k)),
                2  => KVal::Guid(   KData::list(k)),
                4  => KVal::Byte(   KData::list(k)),
                5  => KVal::Short(  KData::list(k)),
                6  => KVal::Int(    KData::list(k)),
                7  => KVal::Long(   KData::list(k)),
                8  => KVal::Real(   KData::list(k)),
                9  => KVal::Float(  KData::list(k)),
                #[cfg(not(feature="unchecked_utf8"))]
                10 => {
                    let s = std::str::from_utf8(k.fetch_slice::<u8>());
                    KVal::String(s.unwrap())
                },
                #[cfg(feature="unchecked_utf8")]
                10 => {
                    let s = std::str::from_utf8_unchecked(k.fetch_slice::<u8>());
                    KVal::String(s)
                },
                11 => KVal::Symbol( KData::list(k)),
                12 => KVal::Timestamp( KData::list(k)),
                13 => KVal::Month( KData::list(k)),
                14 => KVal::Date( KData::list(k)),
                15 => KVal::Datetime( KData::list(k)),
                16 => KVal::Timespan( KData::list(k)),
                17 => KVal::Minute( KData::list(k)),
                18 => KVal::Second( KData::list(k)),
                19 => KVal::Time( KData::list(k)),
                98 => KVal::Table( Box::new(KVal::new(k))),
                99 => {
                    let slice = k.fetch_slice::<&K>();
                    KVal::Dict(   Box::new(KVal::new(slice[0])),
                                  Box::new(KVal::new(slice[1])))
                }
                100 => KVal::Function,
                _ => KVal::Unknown
            }
        }
    }

    pub fn to_k(&self) -> &K  {
        match self {
            KVal::Mixed(ref arr) => kmixed(arr),
            KVal::Bool(KData::Atom(&mut v)) => kbool(v),
            KVal::Bool(KData::List(ref vals)) => klist::<bool>(1, vals),
            KVal::Byte(KData::Atom(&mut v)) => kbyte(v),
            KVal::Byte(KData::List(ref vals)) => klist::<u8>(4, vals),
            KVal::Short(KData::Atom(&mut v)) => kshort(v),
            KVal::Short(KData::List(ref vals)) => klist::<i16>(5, vals),
            KVal::Int(KData::Atom(&mut v)) => kint(v),
            KVal::Int(KData::List(ref vals)) => klist::<i32>(6, vals),
            KVal::Long(KData::Atom(&mut v)) => klong(v),
            KVal::Long(KData::List(ref vals)) => klist::<i64>(7, vals),
            KVal::Real(KData::Atom(&mut v)) => kreal(v),
            KVal::Real(KData::List(ref vals)) => klist::<f32>(8, vals),
            KVal::Float(KData::Atom(&mut v)) => kfloat(v),
            KVal::Float(KData::List(ref vals)) => klist::<f64>(9, vals),
            KVal::Symbol(KData::Atom(ref v)) => ksymbol(v),
            KVal::Symbol(KData::List(ref vals)) => klist::<*const i8>(11, &intern_strings(vals.to_vec())),
            KVal::Dict(k, v) => kdict(&k, &v),
            KVal::String(ref s) => kstring(s),
            ref unknown => {
                println!("{:?}", unknown);
                kerror("NYI")
            }
        }
    }

}

pub fn intern_strings(strs: Vec<String>) -> Vec<*const i8> {
    unsafe {
        strs.into_iter()
            .map(|s| ss(ffi::CString::new(s).unwrap().as_ptr()))
            .collect() 
    }
}

pub fn valid_stream(k: &K) -> bool {
    unsafe { okx(k) == 1 }
}

pub fn deserial(k: &K) -> &K  {
    unsafe { &*d9(k) }
}

pub fn kerror(err: &str) -> &'static K {
    let mut map = ERR_STRS.lock().unwrap();

    let msg = map.entry(err.to_string()).or_insert(ffi::CString::new(err).unwrap());
    let ptr = msg.as_ptr();

    // AFAICT, just returns a null pointer
    unsafe { &*krr(ptr) }
}

pub fn kbool(b: bool) -> &'static K {
    unsafe { &*kb( { if b { 1 } else { 0 } } ) }
}

pub fn kbyte(b: u8) -> &'static K {
    unsafe { &*kg(b as i32) }
}

pub fn kshort(h: i16) -> &'static K {
    unsafe { &*kh(h as i32) }
}

pub fn kint(i: i32) -> &'static K {
    unsafe { &*ki(i) }
}

pub fn klong(j: i64) -> &'static K {
    unsafe { &*kj(j) }
}

pub fn kreal(e: f32) -> &'static K {
    unsafe { &*ke(e as f64) }
}

pub fn kfloat(f: f64) -> &'static K {
    unsafe { &*kf(f) }
}

pub fn kchar(c: char) -> &'static K {
    unsafe { &*kc(c as i32) }
}

pub fn kstring(s: &str) -> &'static K {
     unsafe { &*kpn(ffi::CString::new(s).unwrap().as_ptr(), s.len() as i64) }
}

pub fn ksymbol(s: &str) -> &'static K {
    unsafe { &*(ks(ffi::CString::new(s).unwrap().as_ptr())) }
}

pub fn kvoid() -> *const K {
    ptr::null()
}

pub fn klist<T>(ktype: i32, vals: &[T]) -> &'static K {
    unsafe {
        let k = ktn(ktype, vals.len() as i64);
        let sx = (*k).fetch_slice::<T>();
        assert_eq!(vals.len(), sx.len());
        std::ptr::copy_nonoverlapping(vals.as_ptr(), sx.as_mut_ptr(), vals.len());
        &*k
    }
}

pub fn kdict(keys: &KVal, vals: &KVal) -> &'static K {
    unsafe { &*xD(keys.to_k(), vals.to_k()) }
}

pub fn ktable(dict: KVal) -> &'static K {
    unsafe { &*xT(dict.to_k()) }
}

pub fn kmixed(vals: &[KVal]) -> &'static K {
    let (k, sx);
    unsafe {
        k = &*ktn(0, vals.len() as i64);
        sx = (*k).fetch_slice::<&K>();
    }
    assert_eq!(vals.len(), sx.len());
    for (ix, val) in vals.iter().enumerate() {
        sx[ix] = val.to_k()
    }
    k
}

/* TODO
    pub fn ks(arg1: S) -> *const K;                // create symbol
    pub fn kd(arg1: I) -> *const K;                // create date
    pub fn kz(arg1: F) -> *const K;                // create datetime
    pub fn kt(arg1: I) -> *const K;                // create time
    pub fn ku(arg1: U) -> *const K;                // create guid
    pub fn ka(arg1: I) -> *const K;                // create atom
    pub fn ktn(arg1: I, arg2: J) -> *const K;      // create list
    pub fn knk(arg1: I, ...) -> *const K;          // create mixed list
    pub fn ktj(arg1: I, arg2: J) -> *const K;      // create timestamp
    pub fn kp(arg1: S) -> *const K;                // create string
    pub fn kpn(arg1: S, arg2: J) -> *const K;      // create string length n
    pub fn xT(arg1: K) -> *const K;                // create table from dict
    pub fn xD(arg1: K, arg2: K) -> *const K;       // create dict
    pub fn ktd(arg1: K) -> *const K;               // simple table from keyed table
*/
