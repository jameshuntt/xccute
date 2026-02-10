use crate::command::ShellCommand;

#[derive(Default)]
pub struct IdBuilder {
    pub username: Option<String>,
    pub user_id_only: bool,
    pub group_id_only: bool,
    pub groups: bool,
    pub name: bool,
}

impl IdBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn username(mut self, name: impl Into<String>) -> Self {
        self.username = Some(name.into());
        self
    }

    pub fn user_id_only(mut self) -> Self {
        self.user_id_only = true;
        self
    }

    pub fn group_id_only(mut self) -> Self {
        self.group_id_only = true;
        self
    }

    pub fn groups(mut self) -> Self {
        self.groups = true;
        self
    }

    pub fn name(mut self) -> Self {
        self.name = true;
        self
    }
}

impl ShellCommand for IdBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["id".to_string()];

        if self.user_id_only {
            parts.push("-u".to_string());
        }

        if self.group_id_only {
            parts.push("-g".to_string());
        }

        if self.groups {
            parts.push("-G".to_string());
        }

        if self.name {
            parts.push("-n".to_string());
        }

        if let Some(name) = &self.username {
            parts.push(name.clone());
        }

        parts.join(" ")
    }
}

// let id_cmd = IdBuilder::new()
//     .user_id_only()
//     .name()
//     .username("james")
//     .build();
// // => "id -u -n james"
