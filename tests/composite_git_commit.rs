use std::ffi::{OsStr, OsString};
use std::marker::PhantomData;

use xccute::{
    CompositeArgvPart,
    CompositeShellCommand,
    CompositeShellRoot,
    RootableSubCommand,
    ShellCommand,
};

// -----------------------------------------------------------------------------
// This section represents:
//
// #[derive(CompositeShell)]
// #[shell(program = "git")]
// pub struct Git {
//     #[composite(command)]
//     commit: GitCommit,
// }
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default)]
struct Git;

impl CompositeShellRoot for Git {
    fn program() -> &'static OsStr {
        OsStr::new("git")
    }
}

impl Git {
    fn commit() -> <GitCommit as RootableSubCommand<Git>>::Rooted {
        GitCommit::default().rooted()
    }
}

// -----------------------------------------------------------------------------
// This section represents:
//
// #[derive(SimpleSubCommand, Default)]
// #[subcommand(segment = "commit")]
// pub struct GitCommit {
//     #[builder(method = "message", into)]
//     #[arg(kv = "-m")]
//     message: Option<OsString>,
//
//     #[arg(flag = "-a")]
//     all: bool,
// }
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
struct GitCommit {
    message: Option<OsString>,
    all: bool,
}

#[derive(Debug, Clone)]
struct GitCommitRooted<R>
where
    R: CompositeShellRoot,
{
    inner: GitCommit,
    _root: PhantomData<R>,
}

impl<R> RootableSubCommand<R> for GitCommit
where
    R: CompositeShellRoot,
{
    type Rooted = GitCommitRooted<R>;

    fn rooted(self) -> Self::Rooted {
        GitCommitRooted {
            inner: self,
            _root: PhantomData,
        }
    }
}

impl<R> GitCommitRooted<R>
where
    R: CompositeShellRoot,
{
    fn message(mut self, value: impl Into<OsString>) -> Self {
        self.inner.message = Some(value.into());
        self
    }

    fn all(mut self) -> Self {
        self.inner.all = true;
        self
    }
}

impl CompositeArgvPart for GitCommit {
    fn push_argv_part(&self, out: &mut Vec<OsString>) {
        out.push(OsString::from("commit"));

        if let Some(message) = &self.message {
            out.push(OsString::from("-m"));
            out.push(message.clone());
        }

        if self.all {
            out.push(OsString::from("-a"));
        }
    }
}

impl<R> CompositeShellCommand for GitCommitRooted<R>
where
    R: CompositeShellRoot,
{
    fn program(&self) -> &OsStr {
        R::program()
    }

    fn push_args(&self, out: &mut Vec<OsString>) {
        self.inner.push_argv_part(out);
    }
}

#[test]
fn git_commit_rooted_command_emits_argv() {
    let cmd = Git::commit().message("hello").all();

    assert_eq!(CompositeShellCommand::program(&cmd), OsStr::new("git"));

    assert_eq!(
        cmd.argv(),
        vec![
            OsString::from("commit"),
            OsString::from("-m"),
            OsString::from("hello"),
            OsString::from("-a"),
        ]
    );

    assert_eq!(cmd.build_display(), "git commit -m hello -a");

    // The xccute_contract command can also flow into the existing ShellCommand
    // adapter without falling back to `sh -c`.
    assert_eq!(ShellCommand::build(&cmd), "git commit -m hello -a");
}
