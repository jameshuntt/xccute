// shell_commands/src/util/sort.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone, Default)]
pub struct SortBuilder {
    pub numeric: bool,
    pub reverse: bool,
    pub human_numeric: bool,
    pub ignore_case: bool,
    pub unique: bool,
    pub key: Option<String>,
    pub file: Option<String>,
}

impl SortBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn numeric(mut self) -> Self {
        self.numeric = true;
        self
    }

    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    pub fn human_numeric(mut self) -> Self {
        self.human_numeric = true;
        self
    }

    pub fn ignore_case(mut self) -> Self {
        self.ignore_case = true;
        self
    }

    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }

    pub fn file(mut self, f: impl Into<String>) -> Self {
        self.file = Some(f.into());
        self
    }
}

impl ShellCommand for SortBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["sort".to_string()];

        if self.numeric {
            parts.push("-n".to_string());
        }

        if self.reverse {
            parts.push("-r".to_string());
        }

        if self.human_numeric {
            parts.push("-h".to_string());
        }

        if self.ignore_case {
            parts.push("-f".to_string());
        }

        if self.unique {
            parts.push("-u".to_string());
        }

        if let Some(key) = &self.key {
            parts.push("-k".to_string());
            parts.push(key.clone());
        }

        if let Some(file) = &self.file {
            parts.push(file.clone());
        }

        parts.join(" ")
    }
}
