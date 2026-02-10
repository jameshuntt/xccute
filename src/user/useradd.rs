// shell_commands/src/user/useradd.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone)]
pub struct UserAddBuilder {
    pub username: String,
    pub home_dir: Option<String>,
    pub shell: Option<String>,
    pub group: Option<String>,
    pub uid: Option<u32>,
    pub system: bool,
    pub create_home: bool,
}

impl UserAddBuilder {
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            home_dir: None,
            shell: None,
            group: None,
            uid: None,
            system: false,
            create_home: false,
        }
    }

    pub fn home_dir(mut self, dir: impl Into<String>) -> Self {
        self.home_dir = Some(dir.into());
        self
    }

    pub fn shell(mut self, shell: impl Into<String>) -> Self {
        self.shell = Some(shell.into());
        self
    }

    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn uid(mut self, uid: u32) -> Self {
        self.uid = Some(uid);
        self
    }

    pub fn system(mut self) -> Self {
        self.system = true;
        self
    }

    pub fn create_home(mut self) -> Self {
        self.create_home = true;
        self
    }
}

impl ShellCommand for UserAddBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["useradd".to_string()];
        if self.system {
            parts.push("--system".to_string());
        }
        if self.create_home {
            parts.push("-m".to_string());
        }
        if let Some(ref dir) = self.home_dir {
            parts.push("-d".to_string());
            parts.push(dir.clone());
        }
        if let Some(ref shell) = self.shell {
            parts.push("-s".to_string());
            parts.push(shell.clone());
        }
        if let Some(ref group) = self.group {
            parts.push("-g".to_string());
            parts.push(group.clone());
        }
        if let Some(uid) = self.uid {
            parts.push("-u".to_string());
            parts.push(uid.to_string());
        }
        parts.push(self.username.clone());
        parts.join(" ")
    }
}
