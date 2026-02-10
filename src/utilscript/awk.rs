// shell_commands/src/util/awk.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone)]
pub struct AwkBuilder {
    pub program: String,
    pub file: Option<String>,
    pub field_separator: Option<String>,
    pub var_assignments: Vec<(String, String)>,
}

impl AwkBuilder {
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            file: None,
            field_separator: None,
            var_assignments: vec![],
        }
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn field_separator(mut self, sep: impl Into<String>) -> Self {
        self.field_separator = Some(sep.into());
        self
    }

    pub fn with_var(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.var_assignments.push((name.into(), value.into()));
        self
    }
}

impl ShellCommand for AwkBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["awk".to_string()];
        if let Some(sep) = &self.field_separator {
            parts.push("-F".to_string());
            parts.push(format!("'{}'", sep));
        }

        for (k, v) in &self.var_assignments {
            parts.push(format!("{}={}", k, v));
        }

        parts.push(format!("'{}'", self.program));
        if let Some(file) = &self.file {
            parts.push(file.clone());
        }

        parts.join(" ")
    }
}
