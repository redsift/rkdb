// the difference between the configurations is that api a) links to libkdb.a and b) includes ipc

#[cfg(feature="api")]
#[link(name="kdb")]

extern "C" {
    pub fn ktn(arg1: I, arg2: J) -> *const K;      // create list
    pub fn knk(arg1: I, ...) -> *const K;          // create mixed list
    pub fn ku(arg1: U) -> *const K;                // create guid
    pub fn ka(arg1: I) -> *const K;                // create atom
    pub fn kb(arg1: I) -> *const K;                // create boolean
    pub fn kg(arg1: I) -> *const K;                // create byte
    pub fn kh(arg1: I) -> *const K;                // create short
    pub fn ki(arg1: I) -> *const K;                // create int
    pub fn kj(arg1: J) -> *const K;                // create long
    pub fn ke(arg1: F) -> *const K;                // create real
    pub fn kf(arg1: F) -> *const K;                // create float
    pub fn kc(arg1: I) -> *const K;                // create char
    pub fn ks(arg1: S) -> *const K;                // create symbol
    pub fn kd(arg1: I) -> *const K;                // create date
    pub fn kz(arg1: F) -> *const K;                // create datetime
    pub fn kt(arg1: I) -> *const K;                // create time
    pub fn ktj(arg1: I, arg2: J) -> *const K;      // create timestamp
    pub fn kp(arg1: S) -> *const K;                // create string
    pub fn kpn(arg1: S, arg2: J) -> *const K;      // create string length n
    pub fn xT(arg1: *const K) -> *const K;                // create table from dict
    pub fn xD(arg1: *const K, arg2: *const K) -> *const K;       // create dict
    pub fn ktd(arg1: *const K) -> *const K;               // simple table from keyed table

    pub fn ss(arg1: S) -> S;                // intern a string
    pub fn sn(arg1: S, arg2: I) -> S;       // intern n chars from string

    pub fn ymd(arg1: I, arg2: I, arg3: I) -> I;     // encode year/month/day as int
    pub fn dj(arg1: I) -> I;                        // create date from int

    pub fn setm(arg1: I) -> I;

    // IPC
    pub fn khp(arg1: S, arg2: I) -> I;                      // connect to server
    pub fn khpu(arg1: S, arg2: I, arg3: S) -> I;            // connect with username
    pub fn khpun(arg1: S, arg2: I, arg3: S, arg4: I) -> I;  // connect with username, timeout
    pub fn kclose(arg1: I) -> V;            // close socket
    pub fn k(arg1: I, arg2: S, ...) -> *const K;   // remote execution

    pub fn r1(arg1: K) -> *const K;                // increment ref count
    pub fn r0(arg1: *const K) -> V;                // decrement ref count
    pub fn m9() -> V;                       // garbage collect (?)
    pub fn sd1(arg1: I, arg2: Option<extern "C" fn(arg1: I) -> *const K>) -> *const K; // set callback
    pub fn sd0(arg1: I) -> V;                                            // remove callback
    pub fn sd0x(arg1: I, arg2: I) -> V;  
    
    pub fn dl(f: *mut V, arg1: I) -> *const K;     // dynamic link

    pub fn ja(arg1: *mut K, arg2: *mut V) -> *const K;     // join atom to list
    pub fn js(arg1: *mut K, arg2: S) -> *const K;          // join string to list
    pub fn jk(arg1: *mut K, arg2: K) -> *const K;          // join k obj to list
    pub fn jv(k: *mut K, arg1: K) -> *const K;             // join two lists

    pub fn krr(arg1: S) -> *const K;                       // raise error
    pub fn orr(arg1: S) -> *const K;                       // raise system error
    pub fn dot(arg1: K, arg2: K) -> *const K;      // 'dot' function (apply-over)

    pub fn okx(arg1: *const K) -> I;               // check byte stream valid
    pub fn b9(arg1: I, arg2: *const K) -> *const K;       // serialize object
    pub fn d9(arg1: *const K) -> *const K;                // deserialize byte stream
}