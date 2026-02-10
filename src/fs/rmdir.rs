use crate::command::ShellCommand;

#[derive(Default)]
pub struct RmdirBuilder {
    pub recursive: bool,
    pub verbose: bool,
    pub directory: Option<String>,
}

impl RmdirBuilder {
    pub fn new(directory: impl Into<String>) -> Self {
        Self {
            directory: Some(directory.into()),
            ..Default::default()
        }
    }

    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
}

impl ShellCommand for RmdirBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["rmdir".into()];
        if self.recursive { parts.push("-r".into()); }
        if self.verbose { parts.push("-v".into()); }
        if let Some(directory) = &self.directory {
            parts.push(directory.clone());
        }
        parts.join(" ")
    }
}

// let cmd = RmdirBuilder::new("/path/to/directory")
//     .recursive()
//     .verbose()
//     .build();
// 
// println!("{}", cmd); // → rmdir -r -v /path/to/directory
