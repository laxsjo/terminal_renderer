use crate::input;
use crate::input::InputEvent;
use crate::input::KeyCode;
use crate::INPUT;
// use crossterm::event::KeyCode;
use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::VecDeque;

// This logic could probably be moved to some other place...
pub struct InputManager<'a> {
    input: &'a input::Input,
}

impl<'a> InputManager<'a> {
    pub fn new() -> Self {
        Self { input: &INPUT }
    }

    pub fn tick(&mut self) -> InputStream {
        InputStream::new(self.input.iter().collect())
    }
}

impl<'a> Default for InputManager<'a> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct InputStream {
    events: Vec<InputEvent>,
}

impl InputStream {
    fn new(events: Vec<InputEvent>) -> Self {
        Self { events }
    }

    /// Create a new empty `InputStream`
    pub fn empty() -> Self {
        Self { events: Vec::new() }
    }

    pub fn iter(&self) -> std::slice::Iter<InputEvent> {
        self.events.iter()
    }

    pub fn contains(&self, event: &InputEvent) -> bool {
        self.iter().contains(event)
    }

    pub fn contains_key_code(&self, key_code: KeyCode) -> bool {
        self.iter().any(|event| event.code == key_code)
    }
}
