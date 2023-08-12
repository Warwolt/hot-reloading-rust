use std::ffi::OsStr;

pub struct App {
    _lib: libloading::Library,
    hello: libloading::os::windows::Symbol<fn() -> ()>,
}

impl App {
    pub fn new<P: AsRef<OsStr>>(lib_path: P) -> Self {
        let _lib = unsafe { libloading::Library::new(lib_path).unwrap() };
        let hello = { unsafe { _lib.get::<fn() -> ()>(b"hello").unwrap().into_raw() } };
        App { _lib, hello }
    }

    pub fn hello(&self) {
        (self.hello)()
    }
}
