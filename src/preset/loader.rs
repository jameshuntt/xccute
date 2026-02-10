use crate::ShellCommand;

pub struct Preset {
    pub name: String,
    pub description: Option<String>,
    pub commands: Vec<Box<dyn ShellCommand>>,
}

