use crate::command::ShellCommand;

#[derive(Default)]
pub struct TouchBuilder {
    pub create: bool,
    pub no_create: bool,
    pub date: Option<String>,
    pub reference: Option<String>,
    pub time: Option<String>,
    pub file: Option<String>,
}

impl TouchBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create(mut self) -> Self {
        self.create = true;
        self
    }

    pub fn no_create(mut self) -> Self {
        self.no_create = true;
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn reference(mut self, path: impl Into<String>) -> Self {
        self.reference = Some(path.into());
        self
    }

    pub fn time(mut self, time: impl Into<String>) -> Self {
        self.time = Some(time.into());
        self
    }

    pub fn file(mut self, name: impl Into<String>) -> Self {
        self.file = Some(name.into());
        self
    }
}

impl ShellCommand for TouchBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["touch".into()];
        if self.create { parts.push("--".into()); } // GNU touch always creates by default
        if self.no_create { parts.push("-c".into()); }
        if let Some(date) = &self.date {
            parts.push("-d".into());
            parts.push(date.clone());
        }
        if let Some(ref_path) = &self.reference {
            parts.push("-r".into());
            parts.push(ref_path.clone());
        }
        if let Some(time) = &self.time {
            parts.push("-t".into());
            parts.push(time.clone());
        }
        if let Some(file) = &self.file {
            parts.push(file.clone());
        }

        parts.join(" ")
    }
}

// let cmd = TouchBuilder::new()
//     .file("example.txt")
//     .date("2024-12-25 00:00")
//     .no_create()
//     .build();
// 
// println!("{}", cmd); // → touch -c -d 2024-12-25 00:00 example.txt
