use libloading::os::windows::Symbol as RawSymbol;
use libloading::Library;
use std::env;
use std::path::PathBuf;

pub struct HotLoadedApp {
    lib: Option<Library>,
    orignal_lib_path: PathBuf,
    copied_lib_path: PathBuf,
}

impl HotLoadedApp {
    /// library assumed to exist next to executable
    pub fn new(lib_name: &str) -> Self {
        // Loading a DLL locks it, which prevents rebuilding.
        // Creating a copy to circumvent this.
        let (orignal_lib_path, copied_lib_path) = library_paths(lib_name);
        std::fs::copy(&orignal_lib_path, &copied_lib_path).unwrap();
        let lib = Some(unsafe { Library::new(&copied_lib_path).unwrap() });

        HotLoadedApp {
            lib,
            orignal_lib_path,
            copied_lib_path,
        }
    }

    pub fn reload(&mut self) {
        self.lib = None; // unload DLL
        std::fs::copy(&self.orignal_lib_path, &self.copied_lib_path).unwrap();
        self.lib = Some(unsafe { Library::new(&self.copied_lib_path).unwrap() });
    }

    fn lib(&self) -> &Library {
        self.lib.as_ref().unwrap()
    }
}

impl app::App for HotLoadedApp {
    fn update(&self, state: &mut app::State) {
        (load_symbol::<fn(&Self, &mut app::State) -> ()>(&self.lib(), "update"))(&self, state);
    }

    fn render(&self, state: &app::State) {
        (load_symbol::<fn(&Self, &app::State) -> ()>(&self.lib(), "render"))(&self, state);
    }
}

impl Drop for HotLoadedApp {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.copied_lib_path);
    }
}

fn library_paths(lib_name: &str) -> (PathBuf, PathBuf) {
    let lib_path = exe_dir().join(lib_name);
    let lib_copy_path = exe_dir().join(format!(
        "{}-0.dll",
        lib_path.file_stem().unwrap().to_string_lossy()
    ));

    (lib_path, lib_copy_path)
}

fn exe_dir() -> PathBuf {
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    exe_dir.to_owned()
}

fn load_symbol<T>(lib: &Library, symbol: &str) -> RawSymbol<T> {
    unsafe { lib.get::<T>(symbol.as_bytes()).unwrap().into_raw() }
}
