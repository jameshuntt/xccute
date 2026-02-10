use crate::command::ShellCommand;

#[derive(Default)]
pub struct ScpBuilder {
    pub recursive: bool,
    pub preserve_times: bool,
    pub port: Option<u16>,
    pub identity_file: Option<String>,
    pub quiet: bool,
    pub from: String,
    pub to: String,
}

impl ScpBuilder {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            ..Default::default()
        }
    }

    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }

    pub fn preserve_times(mut self) -> Self {
        self.preserve_times = true;
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn identity_file(mut self, path: impl Into<String>) -> Self {
        self.identity_file = Some(path.into());
        self
    }

    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }
}

impl ShellCommand for ScpBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["scp".to_string()];

        if self.recursive { parts.push("-r".into()); }
        if self.preserve_times { parts.push("-p".into()); }
        if self.quiet { parts.push("-q".into()); }

        if let Some(port) = self.port {
            parts.push("-P".into());
            parts.push(port.to_string());
        }

        if let Some(id_file) = &self.identity_file {
            parts.push("-i".into());
            parts.push(id_file.clone());
        }

        parts.push(self.from.clone());
        parts.push(self.to.clone());

        parts.join(" ")
    }
}

// let scp_cmd = ScpBuilder::new("file.txt", "user@host:/path/")
//     .recursive()
//     .identity_file("~/.ssh/id_rsa")
//     .port(2222)
//     .build();
// // => scp -r -i ~/.ssh/id_rsa -P 2222 file.txt user@host:/path/
