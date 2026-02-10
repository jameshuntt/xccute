// shell_commands/src/util/sed.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone, Default)]
pub struct SedBuilder {
    pub expression: Option<String>,
    pub file: Option<String>,
    pub in_place: bool,
    pub backup_extension: Option<String>,
    pub suppress_output: bool,
}

impl SedBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn expression(mut self, expr: impl Into<String>) -> Self {
        self.expression = Some(expr.into());
        self
    }

    pub fn file(mut self, path: impl Into<String>) -> Self {
        self.file = Some(path.into());
        self
    }

    pub fn in_place(mut self) -> Self {
        self.in_place = true;
        self
    }

    pub fn backup_extension(mut self, ext: impl Into<String>) -> Self {
        self.backup_extension = Some(ext.into());
        self
    }

    pub fn suppress_output(mut self) -> Self {
        self.suppress_output = true;
        self
    }
}

impl ShellCommand for SedBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["sed".to_string()];

        if self.suppress_output {
            parts.push("-n".to_string());
        }

        if self.in_place {
            parts.push("-i".to_string());

            if let Some(ext) = &self.backup_extension {
                parts.push(ext.clone());
            }
        }

        if let Some(expr) = &self.expression {
            parts.push("-e".to_string());
            parts.push(expr.clone());
        }

        if let Some(file) = &self.file {
            parts.push(file.clone());
        }

        parts.join(" ")
    }
}
