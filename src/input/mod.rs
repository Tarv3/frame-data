use glium::glutin::*;
use imgui::{FrameSize, ImGui, ImGuiKey, Ui};

pub mod mouse;
pub mod keyboard;

pub fn get_mouse_delta(event: &Event) -> Option<(f64, f64)> {
    match event {
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => Some(*delta),
            _ => None,
        },
        _ => None,
    }
}

pub fn get_new_aspect(event: &Event) -> Option<f32> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => Some((size.width / size.height) as f32),
            _ => None,
        },
        _ => None,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pressed {
    active: bool,
    modifiers: ModifiersState,
}

impl Pressed {
    pub fn new() -> Pressed {
        Pressed {
            active: false,
            modifiers: Default::default(),
        }
    }

    pub fn set_state(&mut self, state: ElementState, modifiers: ModifiersState) {
        self.active = state == ElementState::Pressed;
        self.modifiers = modifiers;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Button {
    pub pressed: Pressed,
    pub prev_pressed: Pressed,
}

impl Button {
    pub fn new() -> Button {
        Button {
            pressed: Pressed::new(),
            prev_pressed: Pressed::new(),
        }
    }

    pub fn from_input(state: ElementState, mods: ModifiersState) -> Button {
        let mut pressed = Pressed::new();
        pressed.set_state(state, mods);
        let prev_pressed = pressed;

        Button {
            pressed,
            prev_pressed,
        }
    }

    pub fn down(&self) -> bool {
        self.pressed.active
    }

    pub fn changed(&self) -> bool {
        self.pressed.active != self.prev_pressed.active
    }

    pub fn pressed(&self) -> bool {
        self.pressed.active && self.pressed.active != self.prev_pressed.active
    }

    pub fn released(&self) -> bool {
        !self.pressed.active && self.pressed.active != self.prev_pressed.active
    }

    pub fn handle_input(&mut self, state: ElementState, mods: ModifiersState) {
        self.prev_pressed = self.pressed;
        self.pressed.set_state(state, mods);
    }   

    pub fn update(&mut self) {
        self.prev_pressed = self.pressed;
    }
}

pub struct UserInput {
    pub keyboard: keyboard::Keys,
    pub mouse: mouse::Mouse,
}

impl UserInput {
    pub fn new() -> UserInput {
        UserInput {
            keyboard: keyboard::Keys::new(),
            mouse: mouse::Mouse::new(),
        }
    }

    pub fn frame_reset(&mut self) {
        self.mouse.update();
        self.keyboard.reset_chars();
    }

    pub fn update_ui(&self, imgui: &mut ImGui) {
        imgui.set_mouse_pos(self.mouse.position.x, self.mouse.position.y);
        imgui.set_mouse_down(self.mouse.mouse_down());
        imgui.set_mouse_wheel(self.mouse.wheel.delta.map(|(_, y)| y).unwrap_or(0.0));

        for value in self.keyboard.input_chars.iter() {
            imgui.add_input_character(*value);
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::WindowEvent { event, .. } => {
                self.keyboard.handle_window_event(event);
                self.mouse.handle_window_event(event);
            }
            _ => {}
        }
    }
}