use crate::command::ShellCommand;

#[derive(Default)]
pub struct StatBuilder {
    pub path: String,
    pub follow_symlinks: bool,
    pub format: Option<String>,
}

impl StatBuilder {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            ..Default::default()
        }
    }

    pub fn follow_symlinks(mut self) -> Self {
        self.follow_symlinks = true;
        self
    }

    pub fn format(mut self, fmt: impl Into<String>) -> Self {
        self.format = Some(fmt.into());
        self
    }
}

impl ShellCommand for StatBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["stat".to_string()];
        if self.follow_symlinks { parts.push("-L".into()); }
        if let Some(fmt) = &self.format {
            parts.push("--format".into());
            parts.push(fmt.clone());
        }
        parts.push(self.path.clone());
        parts.join(" ")
    }
}

// let stat_cmd = StatBuilder::new("/etc/passwd")
//     .follow_symlinks()
//     .format("%n: %s bytes")
//     .build();
// // => "stat -L --format %n: %s bytes /etc/passwd"
