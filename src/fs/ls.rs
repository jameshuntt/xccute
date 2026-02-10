// src/builder/ls.rs

#[derive(Default)]
pub struct LsBuilder {
    pub long: bool,
    pub all: bool,
    pub human_readable: bool,
    pub path: Option<String>,
}

impl LsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn long(mut self) -> Self {
        self.long = true;
        self
    }

    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    pub fn human_readable(mut self) -> Self {
        self.human_readable = true;
        self
    }

    pub fn target<S: Into<String>>(mut self, path: S) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn build(&self) -> String {
        let mut parts = vec!["ls".to_string()];
        if self.long { parts.push("-l".into()); }
        if self.all { parts.push("-a".into()); }
        if self.human_readable { parts.push("-h".into()); }
        if let Some(path) = &self.path {
            parts.push(path.clone());
        }
        parts.join(" ")
    }
}

// let cmd = Ls::new()
//     .long()
//     .all()
//     .human_readable()
//     .target("/etc/")
//     .build();
// 
// println!("{}", cmd); // "ls -lah /etc/"
