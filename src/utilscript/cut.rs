// shell_commands/src/util/cut.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone, Default)]
pub struct CutBuilder {
    pub delimiter: Option<String>,
    pub fields: Option<String>,
    pub bytes: Option<String>,
    pub characters: Option<String>,
    pub complement: bool,
    pub only_delimited: bool,
    pub file: Option<String>,
}

impl CutBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn delimiter(mut self, d: impl Into<String>) -> Self {
        self.delimiter = Some(d.into());
        self
    }

    pub fn fields(mut self, f: impl Into<String>) -> Self {
        self.fields = Some(f.into());
        self
    }

    pub fn bytes(mut self, b: impl Into<String>) -> Self {
        self.bytes = Some(b.into());
        self
    }

    pub fn characters(mut self, c: impl Into<String>) -> Self {
        self.characters = Some(c.into());
        self
    }

    pub fn complement(mut self) -> Self {
        self.complement = true;
        self
    }

    pub fn only_delimited(mut self) -> Self {
        self.only_delimited = true;
        self
    }

    pub fn file(mut self, f: impl Into<String>) -> Self {
        self.file = Some(f.into());
        self
    }
}

impl ShellCommand for CutBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["cut".to_string()];

        if self.complement {
            parts.push("--complement".to_string());
        }

        if self.only_delimited {
            parts.push("--only-delimited".to_string());
        }

        if let Some(d) = &self.delimiter {
            parts.push("-d".to_string());
            parts.push(d.clone());
        }

        if let Some(f) = &self.fields {
            parts.push("-f".to_string());
            parts.push(f.clone());
        }

        if let Some(b) = &self.bytes {
            parts.push("-b".to_string());
            parts.push(b.clone());
        }

        if let Some(c) = &self.characters {
            parts.push("-c".to_string());
            parts.push(c.clone());
        }

        if let Some(f) = &self.file {
            parts.push(f.clone());
        }

        parts.join(" ")
    }
}
