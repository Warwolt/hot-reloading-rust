trait App {
    fn update(state: &mut State);
    fn render(state: &State);
}

pub struct State {
    pub counter: u32,
}

struct AppImpl;

impl App for AppImpl {
    #[no_mangle]
    fn update(state: &mut State) {
        state.counter += 1;
    }

    #[no_mangle]
    fn render(state: &State) {
        println!("counter = {}", state.counter);
    }
}
