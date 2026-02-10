use crate::command::ShellCommand;

#[derive(Default)]
pub struct PsBuilder {
    pub all: bool,
    pub user: bool,
    pub format: Option<String>,
    pub sort: Option<String>,
    pub custom_args: Vec<String>,
}

impl PsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    pub fn user(mut self) -> Self {
        self.user = true;
        self
    }

    pub fn format(mut self, fmt: impl Into<String>) -> Self {
        self.format = Some(fmt.into());
        self
    }

    pub fn sort(mut self, sort_key: impl Into<String>) -> Self {
        self.sort = Some(sort_key.into());
        self
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.custom_args.push(arg.into());
        self
    }
}

impl ShellCommand for PsBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["ps".to_string()];

        if self.all {
            parts.push("a".into());
        }

        if self.user {
            parts.push("u".into());
        }

        if let Some(fmt) = &self.format {
            parts.push(format!("--format={}", fmt));
        }

        if let Some(sort_key) = &self.sort {
            parts.push(format!("--sort={}", sort_key));
        }

        for arg in &self.custom_args {
            parts.push(arg.clone());
        }

        parts.join(" ")
    }
}

// let ps_cmd = PsBuilder::new()
//     .all()
//     .user()
//     .format("pid,ppid,cmd,%mem,%cpu")
//     .sort("-%cpu")
//     .build();
// // => "ps a u --format=pid,ppid,cmd,%mem,%cpu --sort=-%cpu"
