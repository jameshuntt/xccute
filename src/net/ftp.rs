use crate::command::ShellCommand;

#[derive(Default)]
pub struct FtpBuilder {
    pub host: String,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub script_path: Option<String>,
    pub passive: bool,
    pub no_auto_login: bool,
}

impl FtpBuilder {
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            ..Default::default()
        }
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn script(mut self, path: impl Into<String>) -> Self {
        self.script_path = Some(path.into());
        self
    }

    pub fn passive(mut self) -> Self {
        self.passive = true;
        self
    }

    pub fn no_auto_login(mut self) -> Self {
        self.no_auto_login = true;
        self
    }
}

impl ShellCommand for FtpBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["ftp".to_string()];

        if self.no_auto_login {
            parts.push("-n".into());
        }

        if let Some(path) = &self.script_path {
            parts.push("-s:".into());
            parts.push(path.clone());
        }

        let mut address = self.host.clone();
        if let Some(port) = self.port {
            address.push(':');
            address.push_str(&port.to_string());
        }

        parts.push(address);
        parts.join(" ")
    }
}


// let ftp = FtpBuilder::new("ftp.example.com")
//     .user("anonymous")
//     .no_auto_login()
//     .script("/tmp/ftp_commands.txt")
//     .build();
// // => ftp -n -s:/tmp/ftp_commands.txt ftp.example.com
