use std::collections::HashMap;
use glium::glutin::event::{VirtualKeyCode, MouseButton, ElementState};
use glam::{Vec2, DVec2};

pub struct InputHandler {
    keyboard_key_map: HashMap<VirtualKeyCode, ElementState>,
    mouse_button_map: HashMap<MouseButton, ElementState>,
    mouse_delta: DVec2
}
impl InputHandler {
    pub fn new() -> Self {
        Self {
            keyboard_key_map: HashMap::new(),
            mouse_button_map: HashMap::new(),
            mouse_delta: DVec2::ZERO
        }
    }

    pub fn keyboard_is_pressed(&self, key: &VirtualKeyCode) -> bool {
        match self.keyboard_key_map.get(key) {
            Some(ElementState::Pressed) => true,
            Some(ElementState::Released) => false,
            None => false
        }
    }

    pub fn mouse_is_pressed(&self, button: &MouseButton) -> bool {
        match self.mouse_button_map.get(button) {
            Some(ElementState::Pressed) => true,
            Some(ElementState::Released) => false,
            None => false
        }
    }

    pub fn keyboard_is_released(&self, key: &VirtualKeyCode) -> bool {
        !self.keyboard_is_pressed(key)
    }

    pub fn mouse_is_released(&self, button: &MouseButton) -> bool {
        !self.mouse_is_pressed(button)
    }

    pub fn keyboard_update_key(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.keyboard_key_map.insert(key, state);
    }

    pub fn mouse_update_button(&mut self, button: MouseButton, state: ElementState) {
        self.mouse_button_map.insert(button, state);
    }

    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_delta.as_vec2()
    }

    pub fn add_mouse_delta(&mut self, x_delta: f64, y_delta: f64) {
        self.mouse_delta += DVec2::new(x_delta, y_delta);
    }

    pub fn reset_deltas(&mut self) {
        self.mouse_delta = DVec2::ZERO;
    }
}