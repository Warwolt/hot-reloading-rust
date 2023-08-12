use libloading::os::windows::Symbol as RawSymbol;
use libloading::Library;
use std::env;
use std::path::PathBuf;

// TODO remove the dll copy on drop

pub struct App {
    lib: Option<Library>,
    functions: VTable,
    lib_path: PathBuf,
    lib_copy_path: PathBuf,
}

struct VTable {
    update: RawSymbol<fn() -> ()>,
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

fn load_functions(lib: &Library) -> VTable {
    VTable {
        update: load_symbol(lib, "update"),
    }
}

fn load_symbol<T>(lib: &Library, symbol: &str) -> RawSymbol<T> {
    unsafe { lib.get::<T>(symbol.as_bytes()).unwrap().into_raw() }
}

impl App {
    pub fn new(lib_name: &str) -> Self {
        // To ensure that the DLL can be written to during a rebuild, a copy of
        // the DLL is what's actually loaded
        let (lib_path, lib_copy_path) = get_paths(lib_name);
        std::fs::copy(&lib_path, &lib_copy_path).unwrap();

        let lib = unsafe { Library::new(&lib_copy_path).unwrap() };
        let functions = load_functions(&lib);
        App {
            lib: Some(lib),
            functions,
            lib_path,
            lib_copy_path,
        }
    }

    pub fn update(&self) {
        (self.functions.update)()
    }

    pub fn reload_library(&mut self) {
        // unload library
        self.lib = None;

        // copy rebuilt library
        std::fs::copy(&self.lib_path, &self.lib_copy_path).unwrap();

        // reload library
        self.lib = Some(unsafe { Library::new(&self.lib_copy_path).unwrap() });
        self.functions = load_functions(&self.lib.as_ref().unwrap());
    }
}
