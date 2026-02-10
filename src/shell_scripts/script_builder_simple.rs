use crate::command::ShellCommand;
use std::fs::OpenOptions;
use std::io::{self, Write};

#[derive(Debug, Default, Clone)]
pub struct ScriptBuilder {
    pub file_path: String,
    pub lines: Vec<String>,
}

impl ScriptBuilder {
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
            lines: vec![],
        }
    }

    pub fn add_line(mut self, line: impl Into<String>) -> Self {
        self.lines.push(line.into());
        self
    }

    pub fn add_command<T: ShellCommand>(mut self, command: T) -> Self {
        self.lines.push(command.build());
        self
    }

    pub fn write(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        for line in &self.lines {
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }
}
