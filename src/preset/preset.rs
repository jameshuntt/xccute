// src/preset/preset.rs

use std::collections::HashMap;

pub struct CommandPreset {
    pub name: String,
    pub template: Box<dyn Fn(HashMap<String, String>) -> String + Send + Sync>,
    pub required_args: Vec<String>,
}

impl CommandPreset {
    pub fn run(&self, inputs: HashMap<String, String>) -> String {
        (self.template)(inputs)
    }
}
