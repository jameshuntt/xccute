// shell_commands/src/user/su.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone)]
pub struct SuBuilder {
    pub user: Option<String>,
    pub command: Option<String>,
    pub login: bool,
}

impl SuBuilder {
    pub fn new() -> Self {
        Self {
            user: None,
            command: None,
            login: false,
        }
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn command(mut self, cmd: impl Into<String>) -> Self {
        self.command = Some(cmd.into());
        self
    }

    pub fn login(mut self) -> Self {
        self.login = true;
        self
    }
}

impl ShellCommand for SuBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["su".to_string()];
        if self.login {
            parts.push("-l".to_string());
        }
        if let Some(ref user) = self.user {
            parts.push(user.clone());
        }
        if let Some(ref cmd) = self.command {
            parts.push("-c".to_string());
            parts.push(format!("'{}'", cmd));
        }
        parts.join(" ")
    }
}
