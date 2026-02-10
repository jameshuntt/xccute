use crate::command::ShellCommand;

#[derive(Default)]
pub struct WBuilder {
    pub short: bool,
    pub user: Option<String>,
}

impl WBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn short(mut self) -> Self {
        self.short = true;
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

impl ShellCommand for WBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["w".to_string()];
        if self.short {
            parts.push("-s".into());
        }
        if let Some(user) = &self.user {
            parts.push(user.clone());
        }
        parts.join(" ")
    }
}

// let cmd2 = WBuilder::new().short().user("james").build();
// // => "w -s james"
