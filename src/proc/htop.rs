use crate::command::ShellCommand;

#[derive(Default)]
pub struct HtopBuilder {
    pub user: Option<String>,
    pub sort_key: Option<String>,
    pub tree: bool,
    pub delay: Option<u64>,
    pub no_colors: bool,
    pub no_mouse: bool,
}

impl HtopBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn sort_key(mut self, key: impl Into<String>) -> Self {
        self.sort_key = Some(key.into());
        self
    }

    pub fn tree(mut self) -> Self {
        self.tree = true;
        self
    }

    pub fn delay(mut self, ms: u64) -> Self {
        self.delay = Some(ms);
        self
    }

    pub fn no_colors(mut self) -> Self {
        self.no_colors = true;
        self
    }

    pub fn no_mouse(mut self) -> Self {
        self.no_mouse = true;
        self
    }
}

impl ShellCommand for HtopBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["htop".to_string()];

        if let Some(user) = &self.user {
            parts.push("--user".to_string());
            parts.push(user.clone());
        }

        if let Some(sort_key) = &self.sort_key {
            parts.push("--sort-key".to_string());
            parts.push(sort_key.clone());
        }

        if self.tree {
            parts.push("--tree".to_string());
        }

        if let Some(delay) = &self.delay {
            parts.push("--delay".to_string());
            parts.push(delay.to_string());
        }

        if self.no_colors {
            parts.push("--no-color".to_string());
        }

        if self.no_mouse {
            parts.push("--no-mouse".to_string());
        }

        parts.join(" ")
    }
}

// let htop_cmd = HtopBuilder::new()
//     .user("james")
//     .tree()
//     .delay(100)
//     .build();
// // => "htop --user james --tree --delay 100"
