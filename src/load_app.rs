use libloading::os::windows::Symbol as RawSymbol;
use libloading::Library;
use std::ffi::OsStr;

pub struct App {
    _lib: Library,
    update: RawSymbol<fn() -> ()>,
}

impl App {
    pub fn new<P: AsRef<OsStr>>(lib_path: P) -> Self {
        let _lib = unsafe { Library::new(lib_path).unwrap() };
        let update = { unsafe { _lib.get::<fn() -> ()>(b"update").unwrap().into_raw() } };
        App { _lib, update }
    }

    pub fn update(&self) {
        (self.update)()
    }
}
