use crate::command::ShellCommand;

#[derive(Debug, Default, Clone)]
pub struct CdBuilder {
    pub path: String,
}

impl CdBuilder {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    pub fn join(mut self, subpath: impl Into<String>) -> Self {
        use std::path::PathBuf;
        let mut p = PathBuf::from(self.path);
        p.push(subpath.into());
        self.path = p.to_string_lossy().into_owned();
        self
    }

    pub fn must_exist(self) -> Self {
        // Placeholder: validate existence at runtime?
        self
    }
}

impl ShellCommand for CdBuilder {
    fn build(&self) -> String {
        format!("cd {}", self.path)
    }
}

// let cmd = CdBuilder::new("my_project")
//     .join("src")
//     .build();
// 
// println!("{}", cmd);
// // → cd my_project/src
