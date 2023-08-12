use std::process::{Child, ExitStatus};

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

fn check_command_status(child: &mut Option<Child>) -> Option<ExitStatus> {
    if let Some(child) = child {
        child.try_wait().unwrap()
    } else {
        None
    }
}

fn main() {
    let mut app = load_app::App::new("app.dll");
    let mut state = app::State { counter: 0 };

    let mut prev_tick = std::time::SystemTime::now();
    let mut escape_key = Button::new(winapi::um::winuser::VK_ESCAPE);
    let mut f5_key = Button::new(winapi::um::winuser::VK_F5);

    let mut build_cmd = std::process::Command::new("cargo");
    build_cmd.args(["build", "-p", "app"]);
    let mut build_cmd_invokation: Option<Child> = None;

    'main: loop {
        /* Input */
        let now = std::time::SystemTime::now();

        escape_key.update();
        f5_key.update();

        /* Update */
        if escape_key.pressed_now() {
            break 'main;
        }

        if f5_key.pressed_now() && build_cmd_invokation.is_none() {
            println!("Rebuilding code");
            build_cmd_invokation = Some(build_cmd.spawn().unwrap());
        }

        if now.duration_since(prev_tick).unwrap().as_millis() > 1000 {
            prev_tick = now;
            app.update(&mut state);
            app.render(&state);
        }

        if let Some(status) = check_command_status(&mut build_cmd_invokation) {
            build_cmd_invokation = None;
            if status.success() {
                println!("Done rebuilding");
                app.reload_library();
            } else {
                eprintln!("Build failed");
            }
        }
    }

    println!("Good bye");
}
