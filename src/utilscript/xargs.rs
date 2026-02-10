// src/utilscript/xargs.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone, Default)]
pub struct XargsBuilder {
    pub command: Option<String>,
    pub max_args: Option<usize>,
    pub delimiter: Option<String>,
    pub interactive: bool,
    pub replace_str: Option<String>,
    pub verbose: bool,
}

impl XargsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn command(mut self, cmd: impl Into<String>) -> Self {
        self.command = Some(cmd.into());
        self
    }

    pub fn max_args(mut self, max: usize) -> Self {
        self.max_args = Some(max);
        self
    }

    pub fn delimiter(mut self, delim: impl Into<String>) -> Self {
        self.delimiter = Some(delim.into());
        self
    }

    pub fn replace(mut self, value: impl Into<String>) -> Self {
        self.replace_str = Some(value.into());
        self
    }

    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
}

impl ShellCommand for XargsBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["xargs".to_string()];
        if let Some(cmd) = &self.command {
            parts.push(cmd.clone());
        }
        if let Some(n) = self.max_args {
            parts.push(format!("-n {}", n));
        }
        if let Some(delim) = &self.delimiter {
            parts.push(format!("-d '{}'", delim));
        }
        if let Some(replace) = &self.replace_str {
            parts.push(format!("-I {}", replace));
        }
        if self.interactive {
            parts.push("-p".to_string());
        }
        if self.verbose {
            parts.push("--verbose".to_string());
        }
        parts.join(" ")
    }
}
