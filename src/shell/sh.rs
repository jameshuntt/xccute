use crate::command::ShellCommand;

#[derive(Debug, Clone, Default)]
pub struct ShBuilder {
    pub command: Option<String>,
    pub login: bool,
    pub restricted: bool,
}

impl ShBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn command(mut self, cmd: impl Into<String>) -> Self {
        self.command = Some(cmd.into());
        self
    }

    pub fn login(mut self) -> Self {
        self.login = true;
        self
    }

    pub fn restricted(mut self) -> Self {
        self.restricted = true;
        self
    }
}

impl ShellCommand for ShBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["sh".to_string()];
        if self.login { parts.push("-l".into()); }
        if self.restricted { parts.push("-r".into()); }
        if let Some(cmd) = &self.command {
            parts.push("-c".into());
            parts.push(format!("\"{}\"", cmd));
        }
        parts.join(" ")
    }
}
