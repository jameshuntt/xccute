// shell_commands/src/user/visudo.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone)]
pub struct VisudoBuilder {
    pub check: bool,
    pub file: Option<String>,
    pub quiet: bool,
}

impl VisudoBuilder {
    pub fn new() -> Self {
        Self {
            check: false,
            file: None,
            quiet: false,
        }
    }

    pub fn check_syntax(mut self) -> Self {
        self.check = true;
        self
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }
}

impl ShellCommand for VisudoBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["visudo".to_string()];
        if self.quiet {
            parts.push("-q".to_string());
        }
        if self.check {
            parts.push("-c".to_string());
        }
        if let Some(ref file) = self.file {
            parts.push("-f".to_string());
            parts.push(file.clone());
        }
        parts.join(" ")
    }
}
