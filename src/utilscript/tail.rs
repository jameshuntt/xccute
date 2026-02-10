// shell_commands/src/util/tail.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone, Default)]
pub struct TailBuilder {
    pub file: Option<String>,
    pub lines: Option<usize>,
    pub bytes: Option<usize>,
    pub follow: bool,
    pub quiet: bool,
}

impl TailBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn lines(mut self, lines: usize) -> Self {
        self.lines = Some(lines);
        self
    }

    pub fn bytes(mut self, bytes: usize) -> Self {
        self.bytes = Some(bytes);
        self
    }

    pub fn follow(mut self) -> Self {
        self.follow = true;
        self
    }

    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }
}

impl ShellCommand for TailBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["tail".to_string()];
        if let Some(n) = self.lines {
            parts.push(format!("-n {}", n));
        }
        if let Some(b) = self.bytes {
            parts.push(format!("-c {}", b));
        }
        if self.follow {
            parts.push("-f".to_string());
        }
        if self.quiet {
            parts.push("-q".to_string());
        }
        if let Some(file) = &self.file {
            parts.push(file.clone());
        }
        parts.join(" ")
    }
}
