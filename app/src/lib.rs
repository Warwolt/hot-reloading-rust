pub trait App {
    fn update(&self, state: &mut State);
    fn render(&self, state: &State);
}

pub struct State {
    pub counter: u32,
}

struct AppImpl;

impl App for AppImpl {
    #[no_mangle]
    fn update(&self, state: &mut State) {
        update(state);
    }

    #[no_mangle]
    fn render(&self, state: &State) {
        render(state);
    }
}

pub fn update(state: &mut State) {
    state.counter += 1;
}

pub fn render(state: &State) {
    println!("counter = {}", state.counter);
}
