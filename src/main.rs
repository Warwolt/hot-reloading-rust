use std::ffi::OsStr;

struct App {
    _lib: libloading::Library,
    hello: libloading::os::windows::Symbol<fn() -> ()>,
}

impl App {
    fn new<P: AsRef<OsStr>>(lib_path: P) -> Self {
        let _lib = unsafe { libloading::Library::new(lib_path).unwrap() };
        let hello = { unsafe { _lib.get::<fn() -> ()>(b"hello").unwrap().into_raw() } };
        App { _lib, hello }
    }

    fn hello(&self) {
        (self.hello)()
    }
}

fn main() {
    let app = App::new("./app.dll");
    app.hello();
}
