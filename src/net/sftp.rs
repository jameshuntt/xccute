use crate::command::ShellCommand;

#[derive(Default)]
pub struct SftpBuilder {
    pub user: Option<String>,
    pub host: String,
    pub port: Option<u16>,
    pub identity_file: Option<String>,
    pub batch_file: Option<String>,
    pub interactive: bool,
}

impl SftpBuilder {
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            ..Default::default()
        }
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
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

    pub fn batch_file(mut self, file: impl Into<String>) -> Self {
        self.batch_file = Some(file.into());
        self
    }

    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }
}

impl ShellCommand for SftpBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["sftp".to_string()];

        if let Some(id_file) = &self.identity_file {
            parts.push("-i".into());
            parts.push(id_file.clone());
        }

        if let Some(port) = self.port {
            parts.push("-P".into());
            parts.push(port.to_string());
        }

        if let Some(batch_file) = &self.batch_file {
            parts.push("-b".into());
            parts.push(batch_file.clone());
        }

        if self.interactive {
            parts.push("-C".into()); // Compression (or interactive extensions)
        }

        let destination = match &self.user {
            Some(user) => format!("{}@{}", user, self.host),
            None => self.host.clone(),
        };

        parts.push(destination);
        parts.join(" ")
    }
}

// let sftp_cmd = SftpBuilder::new("example.com")
//     .user("admin")
//     .port(22)
//     .identity_file("~/.ssh/id_rsa")
//     .interactive()
//     .build();
// // => "sftp -i ~/.ssh/id_rsa -P 22 -C admin@example.com"
