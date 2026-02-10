// shell_commands/src/user/sudo.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone)]
pub struct SudoBuilder {
    pub command: String,
    pub user: Option<String>,
    pub group: Option<String>,
    pub preserve_env: bool,
    pub login_shell: bool,
}

impl SudoBuilder {
    pub fn new(cmd: impl Into<String>) -> Self {
        Self {
            command: cmd.into(),
            user: None,
            group: None,
            preserve_env: false,
            login_shell: false,
        }
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn preserve_env(mut self) -> Self {
        self.preserve_env = true;
        self
    }

    pub fn login_shell(mut self) -> Self {
        self.login_shell = true;
        self
    }
}

impl ShellCommand for SudoBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["sudo".to_string()];
        if self.preserve_env {
            parts.push("-E".to_string());
        }
        if self.login_shell {
            parts.push("-i".to_string());
        }
        if let Some(ref user) = self.user {
            parts.push("-u".to_string());
            parts.push(user.clone());
        }
        if let Some(ref group) = self.group {
            parts.push("-g".to_string());
            parts.push(group.clone());
        }
        parts.push(self.command.clone());
        parts.join(" ")
    }
}
