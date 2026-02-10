use crate::ShellCommand;

#[derive(Debug, Clone, Default)]
pub struct ZshBuilder {
    pub command: Option<String>,
    pub interactive: bool,
    pub login: bool,
    pub emacs: bool,
}

impl ZshBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn command(mut self, cmd: impl Into<String>) -> Self {
        self.command = Some(cmd.into());
        self
    }

    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    pub fn login(mut self) -> Self {
        self.login = true;
        self
    }

    pub fn emacs(mut self) -> Self {
        self.emacs = true;
        self
    }
}

impl ShellCommand for ZshBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["zsh".to_string()];
        if self.login { parts.push("-l".into()); }
        if self.interactive { parts.push("-i".into()); }
        if self.emacs { parts.push("-e".into()); }
        if let Some(cmd) = &self.command {
            parts.push("-c".into());
            parts.push(format!("\"{}\"", cmd));
        }
        parts.join(" ")
    }
}
