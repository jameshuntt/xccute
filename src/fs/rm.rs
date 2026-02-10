use crate::command::ShellCommand;

#[derive(Default)]
pub struct RmBuilder {
    pub recursive: bool,
    pub force: bool,
    pub interactive: bool,
    pub verbose: bool,
    pub path: String,
}

impl RmBuilder {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            ..Default::default()
        }
    }

    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }

    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }

    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
}

impl ShellCommand for RmBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["rm".into()];
        if self.recursive { parts.push("-r".into()); }
        if self.force { parts.push("-f".into()); }
        if self.interactive { parts.push("-i".into()); }
        if self.verbose { parts.push("-v".into()); }
        parts.push(self.path.clone());
        parts.join(" ")
    }
}

// let rm = RmBuilder::new("/tmp/myfile")
//     .recursive()
//     .force()
//     .build();
// // "rm -r -f /tmp/myfile"
