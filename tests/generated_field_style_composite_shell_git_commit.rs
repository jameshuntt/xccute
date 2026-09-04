use std::ffi::{OsStr, OsString};

use simple_impl_derive::{CompositeShell, SimpleSubCommand};
use xccute::CompositeShellCommand;

#[derive(CompositeShell, Debug, Clone, Default)]
#[shell(program = "git")]
struct Git {
    #[composite(command = "commit", method = "commit")]
    commit: GitCommit,
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
fn field_style_composite_shell_generates_same_root_command_api() {
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
}
