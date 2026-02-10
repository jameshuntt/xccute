// shell_commands/src/user/passwd.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone)]
pub struct PasswdBuilder {
    pub username: Option<String>,
    pub stdin: bool,
}

impl PasswdBuilder {
    pub fn new() -> Self {
        Self {
            username: None,
            stdin: false,
        }
    }

    pub fn user(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn from_stdin(mut self) -> Self {
        self.stdin = true;
        self
    }
}

impl ShellCommand for PasswdBuilder {
    fn build(&self) -> String {
        let mut cmd = String::new();
        if self.stdin {
            cmd.push_str("echo \"<password>\" | ");
        }
        cmd.push_str("passwd");
        if let Some(ref user) = self.username {
            cmd.push(' ');
            cmd.push_str(user);
        }
        cmd
    }
}
