use crate::command::ShellCommand;

#[derive(Default)]
pub struct MkdirBuilder {
    pub parents: bool,
    pub mode: Option<String>,
    pub verbose: bool,
    pub directory: Option<String>,
}

impl MkdirBuilder {
    pub fn new(directory: impl Into<String>) -> Self {
        Self {
            directory: Some(directory.into()),
            ..Default::default()
        }
    }

    pub fn parents(mut self) -> Self {
        self.parents = true;
        self
    }

    pub fn mode(mut self, mode: impl Into<String>) -> Self {
        self.mode = Some(mode.into());
        self
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
}

impl ShellCommand for MkdirBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["mkdir".into()];
        if self.parents { parts.push("-p".into()); }
        if self.verbose { parts.push("-v".into()); }
        if let Some(mode) = &self.mode {
            parts.push("-m".into());
            parts.push(mode.clone());
        }
        if let Some(directory) = &self.directory {
            parts.push(directory.clone());
        }
        parts.join(" ")
    }
}

// let cmd = MkdirBuilder::new("/path/to/directory")
//     .parents()
//     .mode("755")
//     .verbose()
//     .build();
// 
// println!("{}", cmd); // → mkdir -p -v -m 755 /path/to/directory
