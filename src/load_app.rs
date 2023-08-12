use libloading::os::windows::Symbol as RawSymbol;
use libloading::Library;
use std::env;
use std::path::Path;

pub struct App {
    _lib: Library,
    update: RawSymbol<fn() -> ()>,
}

impl App {
    pub fn new(lib_path: &Path) -> Self {
        assert!(lib_path.is_relative());
        let exe_path = env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();
        let absolute_lib_path = exe_dir.join(lib_path);
        let absolute_lib_copy_path = format!(
            "{}-0.dll",
            absolute_lib_path.file_stem().unwrap().to_string_lossy()
        );
        std::fs::copy(&absolute_lib_path, &absolute_lib_copy_path).unwrap();

        let lib = unsafe { Library::new(&absolute_lib_copy_path).unwrap() };
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
