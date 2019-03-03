use glium::glutin::*;
use std::collections::VecDeque;

pub struct Keys {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub r: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub esc: bool,
    pub input_chars: VecDeque<char>,
}

impl Keys {
    pub fn new() -> Keys {
        Keys {
            w: false,
            a: false,
            s: false,
            d: false,
            r: false,
            ctrl: false,
            shift: false,
            esc: false,
            input_chars: VecDeque::new(),
        }
    }

    pub fn reset_chars(&mut self) {
        self.input_chars.clear();
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                match (input.virtual_keycode, input.state == ElementState::Pressed) {
                    (Some(VirtualKeyCode::A), pressed) => self.a = pressed,
                    (Some(VirtualKeyCode::S), pressed) => self.s = pressed,
                    (Some(VirtualKeyCode::W), pressed) => self.w = pressed,
                    (Some(VirtualKeyCode::D), pressed) => self.d = pressed,
                    (Some(VirtualKeyCode::R), pressed) => self.r = pressed,
                    (Some(VirtualKeyCode::Escape), pressed) => self.esc = pressed,
                    (Some(VirtualKeyCode::LControl), pressed) => self.ctrl = pressed,
                    (Some(VirtualKeyCode::LShift), pressed) => self.shift = pressed,
                    _ => (),
                }
            }
            WindowEvent::ReceivedCharacter(c) => self.input_chars.push_back(*c),
            _ => (),
        }
    }
}