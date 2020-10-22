use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;

const NUM_MOUSE_BUTTONS: usize = 8;
#[allow(dead_code)]
#[derive(Clone, Copy, Default)]
pub struct ButtonState {
    pub pressed: bool,
    pub released: bool,
    pub held: bool,
}
#[derive(Clone, Copy, Default)]
pub struct MouseState {
    pub held: bool,
}
#[derive(Copy, Clone)]
pub struct Input {
    pub key_state: [ButtonState; Scancode::Num as usize],
    pub mouse_state: [MouseState; NUM_MOUSE_BUTTONS],
    pub mouse_pos: (i32, i32),
}

impl Default for Input {
    fn default() -> Self {
        let button = ButtonState {
            pressed: false,
            released: false,
            held: false,
        };
        let mouse = MouseState { held: false };
        Input {
            key_state: [button; Scancode::Num as usize],
            mouse_state: [mouse; NUM_MOUSE_BUTTONS],
            mouse_pos: (0, 0),
        }
    }
}

#[allow(dead_code)]
impl Input {
    pub fn get_key(&self, code: Scancode) -> ButtonState {
        self.key_state[code as usize]
    }
    pub fn get_mouse_pos() {}

    pub fn get_mouse_button(&self, btn: MouseButton) -> MouseState {
        self.mouse_state[btn as usize]
    }

    pub fn new() -> Input {
        let button = ButtonState {
            pressed: false,
            released: false,
            held: false,
        };
        let mouse = MouseState { held: false };
        Input {
            key_state: [button; Scancode::Num as usize],
            mouse_state: [mouse; NUM_MOUSE_BUTTONS],
            mouse_pos: (0, 0),
        }
    }
}
