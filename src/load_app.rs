use libloading::os::windows::Symbol as RawSymbol;
use libloading::Library;
use std::env;
use std::path::PathBuf;

pub struct App {
    lib: Option<Library>,
    update: RawSymbol<fn() -> ()>,
    lib_path: PathBuf,
    lib_copy_path: PathBuf,
}

impl App {
    pub fn new(lib_name: &str) -> Self {
        // To ensure that the DLL can be written to during a rebuild, a copy of
        // the DLL is what's actually loaded
        let (lib_path, lib_copy_path) = get_paths(lib_name);
        std::fs::copy(&lib_path, &lib_copy_path).unwrap();

        let lib = unsafe { Library::new(&lib_copy_path).unwrap() };
        let update = load_symbol(&lib, "update");
        App {
            lib: Some(lib),
            update,
            lib_path,
            lib_copy_path,
        }
    }

    pub fn update(&self) {
        (self.update)()
    }

    pub fn reload_library(&mut self) {
        self.lib = None;
        std::fs::copy(&self.lib_path, &self.lib_copy_path).unwrap();
        self.lib = Some(unsafe { Library::new(&self.lib_copy_path).unwrap() });
        self.update = load_symbol(&self.lib.as_ref().unwrap(), "update");
    }
}

fn get_paths(lib_name: &str) -> (PathBuf, PathBuf) {
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let lib_path = exe_dir.join(lib_name);
    let lib_copy_path = exe_dir.join(format!(
        "{}-0.dll",
        lib_path.file_stem().unwrap().to_string_lossy()
    ));

    (lib_path, lib_copy_path)
}

fn load_symbol<T>(lib: &Library, symbol: &str) -> RawSymbol<T> {
    unsafe { lib.get::<T>(symbol.as_bytes()).unwrap().into_raw() }
}
