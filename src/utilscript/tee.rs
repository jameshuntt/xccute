// src/utilscript/tee.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone, Default)]
pub struct TeeBuilder {
    pub files: Vec<String>,
    pub append: bool,
    pub ignore_interrupts: bool,
}

impl TeeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.files.push(file.into());
        self
    }

    pub fn append(mut self) -> Self {
        self.append = true;
        self
    }

    pub fn ignore_interrupts(mut self) -> Self {
        self.ignore_interrupts = true;
        self
    }
}

impl ShellCommand for TeeBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["tee".to_string()];
        if self.append {
            parts.push("-a".into());
        }
        if self.ignore_interrupts {
            parts.push("-i".into());
        }
        parts.extend(self.files.iter().cloned());
        parts.join(" ")
    }
}
