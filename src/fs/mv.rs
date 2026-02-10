use crate::command::ShellCommand;

#[derive(Default)]
pub struct MvBuilder {
    pub source: String,
    pub destination: String,
    pub force: bool,
    pub interactive: bool,
    pub verbose: bool,
    pub no_clobber: bool,
    pub backup: bool,
}

impl MvBuilder {
    pub fn new(source: impl Into<String>, destination: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            destination: destination.into(),
            ..Default::default()
        }
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

    pub fn no_clobber(mut self) -> Self {
        self.no_clobber = true;
        self
    }

    pub fn backup(mut self) -> Self {
        self.backup = true;
        self
    }
}

impl ShellCommand for MvBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["mv".into()];
        if self.force { parts.push("-f".into()); }
        if self.interactive { parts.push("-i".into()); }
        if self.verbose { parts.push("-v".into()); }
        if self.no_clobber { parts.push("-n".into()); }
        if self.backup { parts.push("-b".into()); }
        parts.push(self.source.clone());
        parts.push(self.destination.clone());
        parts.join(" ")
    }
}

// let mv = MvBuilder::new("old.txt", "new.txt")
//     .interactive()
//     .backup()
//     .build();
// // "mv -i -b old.txt new.txt"
