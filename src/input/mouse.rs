use glm::{Vec2, vec2};
use std::collections::{BTreeMap, btree_map::{Entry, Iter}};
use glium::glutin::{MouseButton, ElementState, WindowEvent, MouseScrollDelta, ModifiersState};
use super::Button;

#[derive(Copy, Clone, Debug)]
pub struct MouseWheel {
    pub delta: Option<(f32, f32)>,
}

impl MouseWheel {
    pub fn new() -> MouseWheel {
        MouseWheel {
            delta: None,
        }
    }

    pub fn reset(&mut self) {
        self.delta = None;
    }

    pub fn handle_wheel_input(&mut self, delta: MouseScrollDelta) {

        match delta {
            MouseScrollDelta::LineDelta(x, y) => self.delta = Some((x, y)),
            MouseScrollDelta::PixelDelta(pos) => self.delta = Some((pos.x as f32, pos.y as f32)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mouse {
    // Stored as top left is (0, 0) bottom right (window dimenisons)
    pub position: Vec2,
    pub prev_pos: Vec2,

    pub left: Button,
    pub right: Button,
    pub middle: Button,
    pub other: BTreeMap<u8, Button>,

    pub wheel: MouseWheel,
}

impl Mouse {
    pub fn new() -> Mouse {
        let position = vec2(0.0, 0.0);
        let button = Button::new();

        Mouse {
            position,
            prev_pos: position,

            left: button,
            right: button,
            middle: button,
            other: BTreeMap::new(),

            wheel: MouseWheel::new(),
        }
    }

    pub fn screen_space_coords(&self, dims: (f32, f32)) -> (f32, f32) {
        let x = (self.position.x / dims.0 - 0.5) * 2.0;
        let y = (-self.position.y / dims.1 + 0.5) * 2.0;

        (x, y) 
    }

    pub fn update(&mut self) {
        self.prev_pos = self.position;

        self.left.update();
        self.right.update();
        self.middle.update();

        for button in self.other.values_mut() {
            button.update();
        }

        self.wheel.reset();
    }

    pub fn handle_move(&mut self, position: impl Into<(f32, f32)>) {
        let (x, y) = position.into();
        self.position = vec2(x, y);
    }

    pub fn handle_mouse_input(&mut self, button: MouseButton, state: ElementState, mods: ModifiersState) {
        match button {
            MouseButton::Left => self.left.handle_input(state, mods),
            MouseButton::Right => self.right.handle_input(state, mods),
            MouseButton::Middle => self.middle.handle_input(state, mods),
            MouseButton::Other(index) => {
                match self.other.entry(index) {
                    Entry::Occupied(ref mut entry) => entry.get_mut().handle_input(state, mods),
                    Entry::Vacant(entry) => {entry.insert(Button::from_input(state, mods));},
                }
            },
        }
    }

    pub fn mouse_down(&self) -> [bool; 5] {
        [
            self.left.down(),
            self.right.down(),
            self.middle.down(),
            false,
            false,
        ]
    }

    pub fn left_mouse_button(&self) -> Button {
        self.left
    }

    pub fn left_mouse_drag(&self) -> Option<(Vec2, ModifiersState)>  {
        let held = self.left.pressed.active && !self.left.changed();

        if held {
            let mods = self.left.pressed.modifiers;
            Some((self.get_movement(), mods))
        }
        else {
            None
        }
    }

    pub fn get_mouse_button(&self, button: MouseButton) -> Option<Button> {
        match button {
            MouseButton::Left => Some(self.left),
            MouseButton::Right => Some(self.right),
            MouseButton::Middle => Some(self.middle),
            MouseButton::Other(index) => {
                match self.other.get(&index) {
                    Some(button) => Some(*button),
                    None => None,
                }
            },
        }
    }

    pub fn get_mouse_button_mut(&mut self, button: MouseButton) -> Option<&mut Button> {
        match button {
            MouseButton::Left => Some(&mut self.left),
            MouseButton::Right => Some(&mut self.right),
            MouseButton::Middle => Some(&mut self.middle),
            MouseButton::Other(index) => {
                match self.other.get_mut(&index) {
                    Some(button) => Some(button),
                    None => None,
                }
            },
        }
    }

    pub fn get_movement(&self) -> Vec2 {
        self.position - self.prev_pos
    }

    pub fn get_button_drag(&self, button: MouseButton) -> Option<(Vec2, ModifiersState)> {
        let button = match self.get_mouse_button(button) {
            Some(button) => button,
            None => return None,
        };

        let held = button.pressed.active && !button.changed();

        if held {
            let mods = button.pressed.modifiers;
            Some((self.get_movement(), mods))
        }
        else {
            None
        }
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                    let (x, y): (f64, f64) = (*position).into();
                    self.handle_move((x as f32, y as f32))
                },
            WindowEvent::MouseInput { state, button, modifiers, ..  } => self.handle_mouse_input(*button, *state, *modifiers),
            WindowEvent::MouseWheel { delta, .. } => self.wheel.handle_wheel_input(*delta),
            _ => (),
        }
    } 

    pub fn button_iter(&self) -> MouseButtonIter {
        MouseButtonIter {
            mouse: &self,
            other_iter: self.other.iter(),
            index: ButtonIndex::Left,
        }
    }
}

enum ButtonIndex {
    Left,
    Right, 
    Middle,
    Other,
}

pub struct MouseButtonIter<'a> {
    mouse: &'a Mouse,
    other_iter: Iter<'a, u8, Button>,
    index: ButtonIndex,
}

impl<'a> Iterator for MouseButtonIter<'a> {
    type Item = (MouseButton, Button);

    fn next(&mut self) -> Option<(MouseButton, Button)> {
        match self.index {
            ButtonIndex::Left => {
                    self.index = ButtonIndex::Right;
                    Some((MouseButton::Left, self.mouse.left))
                }
            ButtonIndex::Right => {
                    self.index = ButtonIndex::Middle;
                    Some((MouseButton::Right, self.mouse.right))
                }
            ButtonIndex::Middle => {
                    self.index = ButtonIndex::Other;
                    Some((MouseButton::Middle, self.mouse.middle))
                }
            ButtonIndex::Other => match self.other_iter.next() {
                    Some((id, button)) => Some((MouseButton::Other(*id), *button)),
                    None => None
                }
        }
    }
}