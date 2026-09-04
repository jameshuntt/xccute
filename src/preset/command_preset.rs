use std::collections::HashMap;

/// A template that renders a command line from named inputs.
pub type PresetTemplate = Box<dyn Fn(HashMap<String, String>) -> String + Send + Sync>;

pub struct CommandPreset {
    pub name: String,
    pub template: PresetTemplate,
    pub required_args: Vec<String>,
}

impl CommandPreset {
    pub fn run(&self, inputs: HashMap<String, String>) -> String {
        (self.template)(inputs)
    }
}
