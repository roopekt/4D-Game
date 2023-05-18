use std::collections::HashMap;
use glium::glutin::event::{VirtualKeyCode, ElementState};
use glam::{Vec2, DVec2};

pub struct InputHandler {
    key_map: HashMap<VirtualKeyCode, ElementState>,
    mouse_delta: DVec2
}
impl InputHandler {
    pub fn new() -> Self {
        Self {
            key_map: HashMap::new(),
            mouse_delta: DVec2::ZERO
        }
    }

    pub fn is_pressed(&self, key: &VirtualKeyCode) -> bool {
        match self.key_map.get(key) {
            Some(state) => *state == ElementState::Pressed,
            None => false
        }
    }

    pub fn is_released(&self, key: &VirtualKeyCode) -> bool {
        !self.is_pressed(key)
    }

    pub fn update_key(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.key_map.insert(key, state);
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