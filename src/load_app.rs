use libloading::os::windows::Symbol as RawSymbol;
use libloading::Library;
use std::ffi::OsStr;

pub struct App {
    _lib: Library,
    update: RawSymbol<fn() -> ()>,
}

impl App {
    pub fn new<P: AsRef<OsStr>>(lib_path: P) -> Self {
        let lib = unsafe { Library::new(lib_path).unwrap() };
        let update = load_symbol(&lib, "update");
        App { _lib: lib, update }
    }

    pub fn update(&self) {
        (self.update)()
    }
}

fn load_symbol<T>(lib: &Library, symbol: &str) -> RawSymbol<T> {
    unsafe { lib.get::<T>(symbol.as_bytes()).unwrap().into_raw() }
}
