use std::sync::{Arc, RwLock};
use crate::command::ShellCommand;

#[derive(Default)]
pub struct LinkBuilder {
    pub symbolic: bool,
    pub verbose: bool,
    pub source: Option<String>,
    pub target: Option<String>,
}

impl LinkBuilder {
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: Some(source.into()),
            target: Some(target.into()),
            ..Default::default()
        }
    }

    pub fn symbolic(mut self) -> Self {
        self.symbolic = true;
        self
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
}

impl ShellCommand for LinkBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["ln".to_string()];
        if self.symbolic {
            parts.push("-s".to_string());
        }
        if self.verbose {
            parts.push("-v".to_string());
        }
        if let Some(src) = &self.source {
            parts.push(src.clone());
        }
        if let Some(dst) = &self.target {
            parts.push(dst.clone());
        }
        parts.join(" ")
    }
}


pub type SharedLinkBuilder = Arc<RwLock<LinkBuilder>>;


let builder = Arc::new(RwLock::new(
    LinkBuilder::new("/path/to/source", "/path/to/target")
        .symbolic()
        .verbose(),
));

{
    let cmd = builder.read().unwrap().build();
    println!("{}", cmd);
}


pub struct AtomicLinkGuard {
    inner: Arc<RwLock<LinkBuilder>>,
    lock_file: String,
}

impl AtomicLinkGuard {
    pub fn try_lock(&self) -> std::io::Result<std::fs::File> {
        use std::fs::OpenOptions;
        use fs2::FileExt;

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.lock_file)?;
        file.lock_exclusive()?; // blocks or errors
        Ok(file)
    }

    pub fn create_link(&self) -> std::io::Result<()> {
        let _guard = self.try_lock()?;
        let builder = self.inner.read().unwrap();
        let cmd = builder.build();
        std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .status()?;
        Ok(())
    }
}


pub enum LinkMode {
    Unsafe(LinkBuilder),
    Safe(AtomicLinkGuard),
}


pub trait ExecutableLink {
    fn execute(&self) -> std::io::Result<()>;
}


impl ExecutableLink for LinkBuilder {
    fn execute(&self) -> std::io::Result<()> {
        let cmd = self.build();
        std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .status()?;
        Ok(())
    }
}

impl ExecutableLink for AtomicLinkGuard {
    fn execute(&self) -> std::io::Result<()> {
        self.create_link()
    }
}


fn create_critical_link() -> std::io::Result<()> {
    let builder = Arc::new(RwLock::new(
        LinkBuilder::new("/src/secure", "/dst/secure")
            .symbolic()
            .verbose(),
    ));

    let guard = AtomicLinkGuard {
        inner: builder,
        lock_file: "/tmp/secure_link.lock".to_string(),
    };

    let link = LinkMode::Safe(guard);
    match link {
        LinkMode::Safe(ref guarded) => guarded.execute(),
        LinkMode::Unsafe(ref basic) => basic.execute(),
    }
}


pub struct LinkSystem;

impl LinkSystem {
    pub fn safe(source: &str, target: &str) -> LinkMode {
        let builder = Arc::new(RwLock::new(
            LinkBuilder::new(source, target).symbolic().verbose(),
        ));
        let lock_file = format!("/tmp/link_{}.lock", target.replace("/", "_"));
        LinkMode::Safe(AtomicLinkGuard { inner: builder, lock_file })
    }

    pub fn unsafe_link(source: &str, target: &str) -> LinkMode {
        LinkMode::Unsafe(LinkBuilder::new(source, target))
    }
}


