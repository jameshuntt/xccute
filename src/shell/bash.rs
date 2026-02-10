use crate::ShellCommand;

#[derive(Debug, Clone, Default)]
pub struct BashBuilder {
    pub command: Option<String>,
    pub exit_on_error: bool,
    pub verbose: bool,
    pub login: bool,
}

impl BashBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn command(mut self, cmd: impl Into<String>) -> Self {
        self.command = Some(cmd.into());
        self
    }

    pub fn exit_on_error(mut self) -> Self {
        self.exit_on_error = true;
        self
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    pub fn login(mut self) -> Self {
        self.login = true;
        self
    }
}

impl ShellCommand for BashBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["bash".to_string()];
        if self.exit_on_error { parts.push("-e".into()); }
        if self.verbose { parts.push("-x".into()); }
        if self.login { parts.push("-l".into()); }
        if let Some(cmd) = &self.command {
            parts.push("-c".into());
            parts.push(format!("\"{}\"", cmd));
        }
        parts.join(" ")
    }
}
