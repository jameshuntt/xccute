use crate::command::ShellCommand;

#[derive(Default)]
pub struct EchoBuilder {
    pub message: Option<String>,
    pub newline: bool,
    pub quoted: bool,
    pub to_file: Option<String>,
    pub append: bool,
}

impl EchoBuilder {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: Some(message.into()),
            newline: true,
            ..Default::default()
        }
    }

    pub fn no_newline(mut self) -> Self {
        self.newline = false;
        self
    }

    pub fn quoted(mut self) -> Self {
        self.quoted = true;
        self
    }

    pub fn to_file(mut self, path: impl Into<String>) -> Self {
        self.to_file = Some(path.into());
        self
    }

    pub fn append(mut self) -> Self {
        self.append = true;
        self
    }
}

impl ShellCommand for EchoBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["echo".to_string()];

        if !self.newline {
            parts.push("-n".to_string());
        }

        if let Some(msg) = &self.message {
            let msg_part = if self.quoted {
                format!("\"{}\"", msg)
            } else {
                msg.clone()
            };
            parts.push(msg_part);
        }

        if let Some(file) = &self.to_file {
            let redir = if self.append { ">>" } else { ">" };
            parts.push(redir.to_string());
            parts.push(file.clone());
        }

        parts.join(" ")
    }
}

// let echo = EchoBuilder::new("Hello, world!")
//     .quoted()
//     .no_newline()
//     .to_file("output.txt")
//     .append()
//     .build();
// 
// println!("{}", echo);
// // → echo -n "Hello, world!" >> output.txt
