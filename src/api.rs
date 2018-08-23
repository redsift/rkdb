use std::error::Error;
use std::fmt;
use std::ffi;
use std::io;
use std::sync::mpsc;
use std::convert;
use kbindings;
use kbindings::*;
use kapi;

#[derive(Debug)]
pub struct KError {
    desc: String,
    pub kind: KErr
}

pub type KResult<T> = Result<T, KError>;

impl Error for KError {
    fn description(&self) -> &str {
        &self.desc
    }
}

impl fmt::Display for KError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Kind: {:?}, Desc: {}", self.kind, self.description())
    }
}

impl KError {
    pub fn new(s: String, kind: KErr) -> Self {
        KError {
            desc: s,
            kind: kind
        }
    }
}

impl convert::From<io::Error> for KError {
    fn from(err: io::Error) -> KError {
        KError {
            desc: err.description().to_string(),
            kind: KErr::IOErr
        }
    }
}

impl convert::From<mpsc::RecvError> for KError {
    fn from(err: mpsc::RecvError) -> KError {
        KError {
            desc: err.description().to_string(),
            kind: KErr::RecvErr
        }
    }
}

#[derive(Debug)]
pub enum KErr {
    ConnectionFailed,
    AuthenticationFailed,
    QueryFailed,
    SocketClosed,
    SocketTimeout,
    IOErr,
    RecvErr,
    SendErr,
    EncodeFailed,
    DecodeFailed,
    CorruptData,
    BadConfig,
    Generic,
    WrongType
}

pub struct Handle {
    host: String,
    port: i32,
    username: String,
    handle: i32
}

impl Handle {
    pub fn connect(host: &str, port: i32, username: &str) -> Result<Handle, Box<Error>> {
        let chost = try!(ffi::CString::new(host)).as_ptr();
        let cuser = try!(ffi::CString::new(username)).as_ptr();
        let handle = match unsafe { kapi::khpu(chost, port, cuser) } {
            h if h < 0 => return Err(Box::new(KError::new("Could not connect".to_string(), KErr::ConnectionFailed))),
            0 => return Err(Box::new(KError::new("Wrong credentials".to_string(), KErr::AuthenticationFailed))),
            h => h
        };
        Ok(Handle {
            host: host.to_string(),
            username: username.to_string(),
            port: port,
            handle: handle
        })
    }

    pub fn query(&self, query: &str) -> Result<KOwned, Box<Error>> {
        let cquery = try!(ffi::CString::new(query)).as_ptr();
        let kptr = unsafe { kapi::k(self.handle, cquery, kvoid()) };
        if kptr.is_null() {
            return Err(Box::new(KError::new("Query failed".to_string(), KErr::QueryFailed)))
        }
        Ok(unsafe { KOwned(&*kptr)} )
    }

    pub fn close(&self) {
        unsafe { kapi::kclose(self.handle) };
    }
}

pub fn serialize(k: &KOwned) -> KResult<KOwned> {
    let ser = unsafe { &*kapi::b9(3, k.0) };
    if kbindings::valid_stream(ser) {
        Ok(KOwned(ser))
    } else {
        Err(KError::new("Invalid serialization".to_string(), KErr::EncodeFailed))
    }
}

pub fn deserialize(ser: &KOwned) -> KResult<KOwned> {
    if kbindings::valid_stream(ser.0) {
        Ok(KOwned(kbindings::deserial(ser.0)))
    } else {
        Err(KError::new("Invalid deserialization".to_string(), KErr::DecodeFailed))
    }
}

