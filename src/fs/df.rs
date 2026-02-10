use crate::command::ShellCommand;

#[derive(Default)]
pub struct DfBuilder {
    pub human_readable: bool,
    pub all: bool,
    pub local: bool,
    pub inodes: bool,
    pub path: Option<String>,
}

impl DfBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn human_readable(mut self) -> Self {
        self.human_readable = true;
        self
    }

    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    pub fn local(mut self) -> Self {
        self.local = true;
        self
    }

    pub fn inodes(mut self) -> Self {
        self.inodes = true;
        self
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl ShellCommand for DfBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["df".into()];
        if self.human_readable { parts.push("-h".into()); }
        if self.all { parts.push("-a".into()); }
        if self.local { parts.push("-l".into()); }
        if self.inodes { parts.push("-i".into()); }
        if let Some(path) = &self.path {
            parts.push(path.clone());
        }
        parts.join(" ")
    }
}

// let df_cmd = DfBuilder::new()
//     .human_readable()
//     .inodes()
//     .path("/")
//     .build();
// // -> "df -h -i /"
