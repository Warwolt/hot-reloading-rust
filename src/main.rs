mod load_app;

struct Button {
    key: winapi::ctypes::c_int,
    was_pressed: bool,
    is_pressed: bool,
}

impl Button {
    fn new(key: winapi::ctypes::c_int) -> Self {
        Button {
            key,
            was_pressed: false,
            is_pressed: false,
        }
    }

    fn update(&mut self) {
        self.was_pressed = self.is_pressed;
        self.is_pressed = key_is_down(self.key);
    }

    fn pressed_now(&self) -> bool {
        !self.was_pressed && self.is_pressed
    }
}

fn key_is_down(key: winapi::ctypes::c_int) -> bool {
    unsafe { winapi::um::winuser::GetKeyState(key) & 1 << 15 != 0 }
}

fn main() {
    let app = load_app::App::new("./app.dll");
    app.hello();

    // loop until escape
    let mut prev_tick = std::time::SystemTime::now();
    let mut escape_key = Button::new(winapi::um::winuser::VK_ESCAPE);
    let mut f5_key = Button::new(winapi::um::winuser::VK_F5);
    'main: loop {
        /* Input */
        let now = std::time::SystemTime::now();

        escape_key.update();
        f5_key.update();

        /* Update */
        if escape_key.pressed_now() {
            break 'main;
        }

        if f5_key.pressed_now() {
            println!("F5 pressed"); // todo, rebuild here
        }

        if now.duration_since(prev_tick).unwrap().as_millis() > 1000 {
            prev_tick = now;
            app.hello(); // make this "update"
        }
    }

    println!("Good bye");
}
