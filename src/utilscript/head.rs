// shell_commands/src/util/head.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone, Default)]
pub struct HeadBuilder {
    pub lines: Option<u32>,
    pub bytes: Option<u32>,
    pub quiet: bool,
    pub verbose: bool,
    pub file: Option<String>,
}

impl HeadBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lines(mut self, n: u32) -> Self {
        self.lines = Some(n);
        self
    }

    pub fn bytes(mut self, b: u32) -> Self {
        self.bytes = Some(b);
        self
    }

    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    pub fn file(mut self, f: impl Into<String>) -> Self {
        self.file = Some(f.into());
        self
    }
}

impl ShellCommand for HeadBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["head".to_string()];

        if let Some(n) = self.lines {
            parts.push("-n".to_string());
            parts.push(n.to_string());
        }

        if let Some(b) = self.bytes {
            parts.push("-c".to_string());
            parts.push(b.to_string());
        }

        if self.quiet {
            parts.push("-q".to_string());
        }

        if self.verbose {
            parts.push("-v".to_string());
        }

        if let Some(f) = &self.file {
            parts.push(f.clone());
        }

        parts.join(" ")
    }
}
