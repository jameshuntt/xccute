// shell_commands/src/user/groupadd.rs

use crate::command::ShellCommand;

#[derive(Debug, Clone)]
pub struct GroupAddBuilder {
    pub groupname: String,
    pub gid: Option<u32>,
    pub system: bool,
}

impl GroupAddBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            groupname: name.into(),
            gid: None,
            system: false,
        }
    }

    pub fn gid(mut self, gid: u32) -> Self {
        self.gid = Some(gid);
        self
    }

    pub fn system(mut self) -> Self {
        self.system = true;
        self
    }
}

impl ShellCommand for GroupAddBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["groupadd".to_string()];
        if self.system {
            parts.push("--system".to_string());
        }
        if let Some(gid) = self.gid {
            parts.push("-g".to_string());
            parts.push(gid.to_string());
        }
        parts.push(self.groupname.clone());
        parts.join(" ")
    }
}
