use libloading::os::windows::Symbol as RawSymbol;
use libloading::Library;
use std::env;
use std::path::{Path, PathBuf};

pub struct App {
    _lib: Library,
    update: RawSymbol<fn() -> ()>,
}

impl App {
    pub fn new(lib_path: &Path) -> Self {
        let (lib_path, lib_copy_path) = get_paths(lib_path);
        std::fs::copy(&lib_path, &lib_copy_path).unwrap();

        let lib = unsafe { Library::new(&lib_copy_path).unwrap() };
        let update = load_symbol(&lib, "update");
        App { _lib: lib, update }
    }

    pub fn update(&self) {
        (self.update)()
    }
}

fn get_paths(relative_lib_path: &Path) -> (PathBuf, PathBuf) {
    assert!(relative_lib_path.is_relative());
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let lib_path = exe_dir.join(relative_lib_path);
    let lib_copy_path = exe_dir.join(format!(
        "{}-0.dll",
        lib_path.file_stem().unwrap().to_string_lossy()
    ));

    (lib_path, lib_copy_path)
}

fn load_symbol<T>(lib: &Library, symbol: &str) -> RawSymbol<T> {
    unsafe { lib.get::<T>(symbol.as_bytes()).unwrap().into_raw() }
}
