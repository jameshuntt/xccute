use crate::command::ShellCommand;

#[derive(Default)]
pub struct DuBuilder {
    pub summarize: bool,
    pub human_readable: bool,
    pub apparent_size: bool,
    pub all: bool,
    pub max_depth: Option<u32>,
    pub path: Option<String>,
}

impl DuBuilder {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: Some(path.into()),
            ..Default::default()
        }
    }

    pub fn summarize(mut self) -> Self {
        self.summarize = true;
        self
    }

    pub fn human_readable(mut self) -> Self {
        self.human_readable = true;
        self
    }

    pub fn apparent_size(mut self) -> Self {
        self.apparent_size = true;
        self
    }

    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    pub fn max_depth(mut self, depth: u32) -> Self {
        self.max_depth = Some(depth);
        self
    }
}

impl ShellCommand for DuBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["du".into()];
        if self.summarize { parts.push("-s".into()); }
        if self.human_readable { parts.push("-h".into()); }
        if self.apparent_size { parts.push("--apparent-size".into()); }
        if self.all { parts.push("-a".into()); }
        if let Some(depth) = self.max_depth {
            parts.push(format!("--max-depth={}", depth));
        }
        if let Some(path) = &self.path {
            parts.push(path.clone());
        }
        parts.join(" ")
    }
}

// let du_cmd = DuBuilder::new("/var")
//     .human_readable()
//     .summarize()
//     .max_depth(1)
//     .build();
// // -> "du -s -h --max-depth=1 /var"
