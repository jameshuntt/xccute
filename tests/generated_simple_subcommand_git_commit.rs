
use std::ffi::{OsStr, OsString};

use simple_impl_derive::SimpleSubCommand;
use xccute::{CompositeShellCommand, CompositeShellRoot};

#[derive(Debug, Clone, Copy, Default)]
struct Git;

impl CompositeShellRoot for Git {
    fn program() -> &'static OsStr {
        OsStr::new("git")
    }
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "commit")]
struct GitCommit {
    #[builder(method = "message", into)]
    #[arg(kv = "-m")]
    message: Option<OsString>,

    #[arg(flag = "-a")]
    all: bool,
}

#[test]
fn generated_simple_subcommand_matches_manual_git_commit_target() {
    let cmd = GitCommit::default().rooted::<Git>().message("hello").all();

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
}
