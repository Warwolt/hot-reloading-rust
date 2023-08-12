fn main() {
    let app_dll = unsafe { libloading::Library::new("./app.dll").unwrap() };
    let hello = unsafe { app_dll.get::<fn() -> ()>(b"hello").unwrap() };
    hello();
}
