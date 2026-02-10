use crate::command::ShellCommand;

#[derive(Default)]
pub struct RsyncBuilder {
    pub archive: bool,
    pub verbose: bool,
    pub recursive: bool,
    pub compress: bool,
    pub delete: bool,
    pub dry_run: bool,
    pub human_readable: bool,
    pub progress: bool,
    pub preserve_times: bool,
    pub preserve_permissions: bool,
    pub source: String,
    pub destination: String,
}

impl RsyncBuilder {
    pub fn new<S: Into<String>>(source: S, destination: S) -> Self {
        Self {
            source: source.into(),
            destination: destination.into(),
            ..Default::default()
        }
    }

    pub fn archive(mut self) -> Self {
        self.archive = true;
        self
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }

    pub fn compress(mut self) -> Self {
        self.compress = true;
        self
    }

    pub fn delete(mut self) -> Self {
        self.delete = true;
        self
    }

    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }

    pub fn human_readable(mut self) -> Self {
        self.human_readable = true;
        self
    }

    pub fn progress(mut self) -> Self {
        self.progress = true;
        self
    }

    pub fn preserve_times(mut self) -> Self {
        self.preserve_times = true;
        self
    }

    pub fn preserve_permissions(mut self) -> Self {
        self.preserve_permissions = true;
        self
    }
}

impl ShellCommand for RsyncBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["rsync".to_string()];

        if self.archive { parts.push("-a".into()); }
        if self.verbose { parts.push("-v".into()); }
        if self.recursive { parts.push("-r".into()); }
        if self.compress { parts.push("-z".into()); }
        if self.delete { parts.push("--delete".into()); }
        if self.dry_run { parts.push("--dry-run".into()); }
        if self.human_readable { parts.push("-h".into()); }
        if self.progress { parts.push("--progress".into()); }
        if self.preserve_times { parts.push("-t".into()); }
        if self.preserve_permissions { parts.push("-p".into()); }

        parts.push(self.source.clone());
        parts.push(self.destination.clone());

        parts.join(" ")
    }
}

// let sync_cmd = RsyncBuilder::new("src/", "user@host:/dest/")
//     .archive()
//     .compress()
//     .progress()
//     .build();
// // => rsync -a -z --progress src/ user@host:/dest/
