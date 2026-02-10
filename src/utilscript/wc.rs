// shell_commands/src/util/wc.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone, Default)]
pub struct WcBuilder {
    pub file: Option<String>,
    pub bytes: bool,
    pub chars: bool,
    pub lines: bool,
    pub words: bool,
    pub max_line_length: bool,
}

impl WcBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn bytes(mut self) -> Self {
        self.bytes = true;
        self
    }

    pub fn chars(mut self) -> Self {
        self.chars = true;
        self
    }

    pub fn lines(mut self) -> Self {
        self.lines = true;
        self
    }

    pub fn words(mut self) -> Self {
        self.words = true;
        self
    }

    pub fn max_line_length(mut self) -> Self {
        self.max_line_length = true;
        self
    }
}

impl ShellCommand for WcBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["wc".to_string()];
        if self.bytes {
            parts.push("-c".to_string());
        }
        if self.chars {
            parts.push("-m".to_string());
        }
        if self.lines {
            parts.push("-l".to_string());
        }
        if self.words {
            parts.push("-w".to_string());
        }
        if self.max_line_length {
            parts.push("-L".to_string());
        }
        if let Some(file) = &self.file {
            parts.push(file.clone());
        }
        parts.join(" ")
    }
}
