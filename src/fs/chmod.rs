use crate::command::ShellCommand;

#[derive(Default)]
pub struct ChmodBuilder {
    pub mode: String,
    pub target: String,
    pub recursive: bool,
    pub reference: Option<String>,
}

impl ChmodBuilder {
    pub fn new(mode: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            mode: mode.into(),
            target: target.into(),
            ..Default::default()
        }
    }

    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }

    pub fn reference(mut self, ref_file: impl Into<String>) -> Self {
        self.reference = Some(ref_file.into());
        self
    }
}

impl ShellCommand for ChmodBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["chmod".to_string()];
        if self.recursive { parts.push("-R".into()); }

        if let Some(ref_file) = &self.reference {
            parts.push("--reference".into());
            parts.push(ref_file.clone());
        }

        parts.push(self.mode.clone());
        parts.push(self.target.clone());

        parts.join(" ")
    }
}

// let chmod_cmd = ChmodBuilder::new("755", "/usr/bin/myscript")
//     .recursive()
//     .build();
// // => "chmod -R 755 /usr/bin/myscript"
