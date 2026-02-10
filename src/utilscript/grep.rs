use crate::command::ShellCommand;

#[derive(Debug, Clone, Default)]
pub struct GrepBuilder {
    pub pattern: String,
    pub ignore_case: bool,
    pub recursive: bool,
    pub line_number: bool,
    pub invert_match: bool,
    pub word_regexp: bool,
    pub extended_regex: bool,
    pub files: Vec<String>,
}

impl GrepBuilder {
    pub fn new(pattern: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into(),
            ..Default::default()
        }
    }

    pub fn ignore_case(mut self) -> Self {
        self.ignore_case = true;
        self
    }

    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }

    pub fn line_number(mut self) -> Self {
        self.line_number = true;
        self
    }

    pub fn invert_match(mut self) -> Self {
        self.invert_match = true;
        self
    }

    pub fn word_regexp(mut self) -> Self {
        self.word_regexp = true;
        self
    }

    pub fn extended_regex(mut self) -> Self {
        self.extended_regex = true;
        self
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.files.push(file.into());
        self
    }

    pub fn files(mut self, files: Vec<impl Into<String>>) -> Self {
        for f in files {
            self.files.push(f.into());
        }
        self
    }
}

impl ShellCommand for GrepBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["grep".to_string()];

        if self.ignore_case {
            parts.push("-i".into());
        }
        if self.recursive {
            parts.push("-r".into());
        }
        if self.line_number {
            parts.push("-n".into());
        }
        if self.invert_match {
            parts.push("-v".into());
        }
        if self.word_regexp {
            parts.push("-w".into());
        }
        if self.extended_regex {
            parts.push("-E".into());
        }

        parts.push(self.pattern.clone());

        parts.extend(self.files.clone());

        parts.join(" ")
    }
}
