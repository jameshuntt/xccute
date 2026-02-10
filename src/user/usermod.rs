// shell_commands/src/user/usermod.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone)]
pub struct UserModBuilder {
    pub username: String,
    pub new_name: Option<String>,
    pub home_dir: Option<String>,
    pub shell: Option<String>,
    pub group: Option<String>,
    pub append_group: Option<String>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub move_home: bool,
    pub lock: bool,
    pub unlock: bool,
}

impl UserModBuilder {
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            new_name: None,
            home_dir: None,
            shell: None,
            group: None,
            append_group: None,
            uid: None,
            gid: None,
            move_home: false,
            lock: false,
            unlock: false,
        }
    }

    pub fn rename(mut self, new_name: impl Into<String>) -> Self {
        self.new_name = Some(new_name.into());
        self
    }

    pub fn home_dir(mut self, dir: impl Into<String>) -> Self {
        self.home_dir = Some(dir.into());
        self
    }

    pub fn move_home(mut self) -> Self {
        self.move_home = true;
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

    pub fn append_group(mut self, group: impl Into<String>) -> Self {
        self.append_group = Some(group.into());
        self
    }

    pub fn uid(mut self, uid: u32) -> Self {
        self.uid = Some(uid);
        self
    }

    pub fn gid(mut self, gid: u32) -> Self {
        self.gid = Some(gid);
        self
    }

    pub fn lock(mut self) -> Self {
        self.lock = true;
        self
    }

    pub fn unlock(mut self) -> Self {
        self.unlock = true;
        self
    }
}

impl ShellCommand for UserModBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["usermod".to_string()];
        if let Some(name) = &self.new_name {
            parts.push("-l".into());
            parts.push(name.clone());
        }
        if let Some(dir) = &self.home_dir {
            parts.push("-d".into());
            parts.push(dir.clone());
        }
        if self.move_home {
            parts.push("-m".into());
        }
        if let Some(shell) = &self.shell {
            parts.push("-s".into());
            parts.push(shell.clone());
        }
        if let Some(group) = &self.group {
            parts.push("-g".into());
            parts.push(group.clone());
        }
        if let Some(append) = &self.append_group {
            parts.push("-a".into());
            parts.push("-G".into());
            parts.push(append.clone());
        }
        if let Some(uid) = self.uid {
            parts.push("-u".into());
            parts.push(uid.to_string());
        }
        if let Some(gid) = self.gid {
            parts.push("-g".into());
            parts.push(gid.to_string());
        }
        if self.lock {
            parts.push("-L".into());
        }
        if self.unlock {
            parts.push("-U".into());
        }
        parts.push(self.username.clone());
        parts.join(" ")
    }
}
