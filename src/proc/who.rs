use crate::command::ShellCommand;

#[derive(Default)]
pub struct WhoBuilder {
    pub all: bool,
    pub count: bool,
    pub heading: bool,
}

impl WhoBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    pub fn count(mut self) -> Self {
        self.count = true;
        self
    }

    pub fn heading(mut self) -> Self {
        self.heading = true;
        self
    }
}

impl ShellCommand for WhoBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["who".to_string()];
        if self.all {
            parts.push("-a".into());
        }
        if self.count {
            parts.push("-q".into());
        }
        if self.heading {
            parts.push("-H".into());
        }
        parts.join(" ")
    }
}

// let cmd3 = WhoBuilder::new().all().heading().build();
// // => "who -a -H"
