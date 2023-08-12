mod load_app;

fn key_is_down(key: winapi::ctypes::c_int) -> bool {
    unsafe { winapi::um::winuser::GetKeyState(key) & 1 << 15 != 0 }
}

fn main() {
    let app = load_app::App::new("./app.dll");
    app.hello();

    // loop until escape
    let mut prev_tick = std::time::SystemTime::now();
    let mut escape_pressed = false;
    let mut f5_pressed = false;
    'main: loop {
        /* Input */
        let now = std::time::SystemTime::now();

        let escape_pressed_prev = escape_pressed;
        escape_pressed = key_is_down(winapi::um::winuser::VK_ESCAPE);
        let escape_pressed_now = !escape_pressed_prev && escape_pressed;

        let f5_pressed_prev = f5_pressed;
        f5_pressed = key_is_down(winapi::um::winuser::VK_F5);
        let f5_pressed_now = !f5_pressed_prev && f5_pressed;

        /* Update */
        if escape_pressed_now {
            break 'main;
        }

        if f5_pressed_now {
            println!("F5 pressed"); // todo, rebuild here
        }

        if now.duration_since(prev_tick).unwrap().as_millis() > 1000 {
            prev_tick = now;
            app.hello(); // make this "update"
        }
    }

    println!("Good bye");
}
