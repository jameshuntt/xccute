// src/preset/registry.rs
use std::collections::HashMap;
use super::preset::CommandPreset;

#[derive(Default)]
pub struct CommandRegistry {
    presets: HashMap<String, CommandPreset>,
}

impl CommandRegistry {
    pub fn register(&mut self, preset: CommandPreset) {
        self.presets.insert(preset.name.clone(), preset);
    }

    pub fn get(&self, name: &str) -> Option<&CommandPreset> {
        self.presets.get(name)
    }
}
