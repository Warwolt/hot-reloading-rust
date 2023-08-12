mod load_app;

fn main() {
    let app = load_app::App::new("./app.dll");
    app.hello();
}
