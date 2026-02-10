use crate::command::ShellCommand;

#[derive(Default)]
pub struct PgrepBuilder {
    pub pattern: Option<String>,
    pub full: bool,
    pub newest: bool,
    pub oldest: bool,
    pub user: Option<String>,
    pub group: Option<String>,
    pub exact: bool,
}

impl PgrepBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pattern(mut self, pat: impl Into<String>) -> Self {
        self.pattern = Some(pat.into());
        self
    }

    pub fn full(mut self) -> Self {
        self.full = true;
        self
    }

    pub fn newest(mut self) -> Self {
        self.newest = true;
        self
    }

    pub fn oldest(mut self) -> Self {
        self.oldest = true;
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn exact(mut self) -> Self {
        self.exact = true;
        self
    }
}

impl ShellCommand for PgrepBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["pgrep".to_string()];

        if self.full {
            parts.push("-f".to_string());
        }

        if self.newest {
            parts.push("-n".to_string());
        }

        if self.oldest {
            parts.push("-o".to_string());
        }

        if self.exact {
            parts.push("-x".to_string());
        }

        if let Some(user) = &self.user {
            parts.push("-u".to_string());
            parts.push(user.clone());
        }

        if let Some(group) = &self.group {
            parts.push("-G".to_string());
            parts.push(group.clone());
        }

        if let Some(pat) = &self.pattern {
            parts.push(pat.clone());
        }

        parts.join(" ")
    }
}

// let pgrep_cmd = PgrepBuilder::new()
//     .full()
//     .user("james")
//     .pattern("nginx")
//     .build();
// // => "pgrep -f -u james nginx"
