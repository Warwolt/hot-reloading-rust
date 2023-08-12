pub struct HotLoadedApp;

impl HotLoadedApp {
    pub fn new() -> Self {
        HotLoadedApp
    }
}

impl app::App for HotLoadedApp {
    fn update(&self, state: &mut app::State) {
        app::update(state);
    }

    fn render(&self, state: &app::State) {
        app::render(state);
    }
}
