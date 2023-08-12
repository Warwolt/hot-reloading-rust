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
        let absolute_lib_path = absolute_lib_path(lib_path);
        let absolute_lib_copy_path = absolute_lib_copy_path(&absolute_lib_path);
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

fn absolute_lib_path(relative_lib_path: &Path) -> PathBuf {
    assert!(relative_lib_path.is_relative());
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    exe_dir.join(relative_lib_path)
}

fn absolute_lib_copy_path(absolute_lib_path: &Path) -> String {
    assert!(absolute_lib_path.is_absolute());
    let absolute_lib_copy_path = format!(
        "{}-0.dll",
        absolute_lib_path.file_stem().unwrap().to_string_lossy()
    );

    absolute_lib_copy_path
}
