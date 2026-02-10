use crate::command::ShellCommand;

#[derive(Default)]
pub struct TelnetBuilder {
    pub host: String,
    pub port: Option<u16>,
    pub timeout: Option<u64>,
    pub escape_char: Option<char>,
}

impl TelnetBuilder {
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

    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout = Some(seconds);
        self
    }

    pub fn escape_char(mut self, c: char) -> Self {
        self.escape_char = Some(c);
        self
    }
}

impl ShellCommand for TelnetBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["telnet".to_string()];

        if let Some(timeout) = self.timeout {
            parts.push("-t".into());
            parts.push(timeout.to_string());
        }

        if let Some(escape_char) = self.escape_char {
            parts.push("-e".into());
            parts.push(escape_char.to_string());
        }

        parts.push(self.host.clone());

        if let Some(port) = self.port {
            parts.push(port.to_string());
        }

        parts.join(" ")
    }
}

// let telnet_cmd = TelnetBuilder::new("towel.blinkenlights.nl")
//     .port(23)
//     .escape_char('^')
//     .build();
// // => "telnet -e ^ towel.blinkenlights.nl 23"
