pub struct State {
    pub counter: u32,
}

#[no_mangle]
pub fn update(state: &mut State) {
    state.counter += 1;
}

#[no_mangle]
pub fn render(state: &State) {
    println!("counter = {}", state.counter);
}
