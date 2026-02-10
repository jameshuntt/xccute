use crate::command::ShellCommand;

#[derive(Default)]
pub struct FileBuilder {
    pub filename: String,
    pub mime: bool,
    pub brief: bool,
    pub dereference: bool,
    pub special_files: bool,
}

impl FileBuilder {
    pub fn new(filename: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            ..Default::default()
        }
    }

    pub fn mime(mut self) -> Self {
        self.mime = true;
        self
    }

    pub fn brief(mut self) -> Self {
        self.brief = true;
        self
    }

    pub fn dereference(mut self) -> Self {
        self.dereference = true;
        self
    }

    pub fn special_files(mut self) -> Self {
        self.special_files = true;
        self
    }
}

impl ShellCommand for FileBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["file".to_string()];
        if self.mime { parts.push("--mime-type".into()); }
        if self.brief { parts.push("--brief".into()); }
        if self.dereference { parts.push("--dereference".into()); }
        if self.special_files { parts.push("--special-files".into()); }
        parts.push(self.filename.clone());
        parts.join(" ")
    }
}

// let file_cmd = FileBuilder::new("archive.tar.gz")
//     .mime()
//     .brief()
//     .build();
// // => "file --mime-type --brief archive.tar.gz"
