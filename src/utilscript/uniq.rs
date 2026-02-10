// shell_commands/src/util/uniq.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone, Default)]
pub struct UniqBuilder {
    pub input: Option<String>,
    pub count: bool,
    pub repeated: bool,
    pub unique_only: bool,
    pub ignore_case: bool,
    pub skip_fields: Option<usize>,
    pub skip_chars: Option<usize>,
}

impl UniqBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.input = Some(input.into());
        self
    }

    pub fn count(mut self) -> Self {
        self.count = true;
        self
    }

    pub fn repeated(mut self) -> Self {
        self.repeated = true;
        self
    }

    pub fn unique_only(mut self) -> Self {
        self.unique_only = true;
        self
    }

    pub fn ignore_case(mut self) -> Self {
        self.ignore_case = true;
        self
    }

    pub fn skip_fields(mut self, n: usize) -> Self {
        self.skip_fields = Some(n);
        self
    }

    pub fn skip_chars(mut self, n: usize) -> Self {
        self.skip_chars = Some(n);
        self
    }
}

impl ShellCommand for UniqBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["uniq".to_string()];
        if self.count {
            parts.push("-c".to_string());
        }
        if self.repeated {
            parts.push("-d".to_string());
        }
        if self.unique_only {
            parts.push("-u".to_string());
        }
        if self.ignore_case {
            parts.push("-i".to_string());
        }
        if let Some(n) = self.skip_fields {
            parts.push(format!("-f {}", n));
        }
        if let Some(n) = self.skip_chars {
            parts.push(format!("-s {}", n));
        }
        if let Some(input) = &self.input {
            parts.push(input.clone());
        }
        parts.join(" ")
    }
}
