use std::ffi::{OsStr, OsString};

use simple_impl_derive::{CompositeShell, SimpleSubCommand};
use xccute::CompositeShellCommand;

#[derive(CompositeShell, Debug, Clone, Copy, Default)]
#[shell(program = "git")]
#[composite(command = "clone", method = "clone_repo", ty = GitClone, init = "url")]
struct Git;

#[derive(SimpleSubCommand, Debug, Clone)]
#[subcommand(segment = "clone")]
struct GitClone {
    #[builder(init_required, into)]
    #[arg(positional)]
    url: OsString,
}

#[test]
fn generated_composite_shell_root_command_can_forward_init_args() {
    let cmd = Git::clone_repo("https://github.com/me/repo.git");

    assert_eq!(CompositeShellCommand::program(&cmd), OsStr::new("git"));
    assert_eq!(
        cmd.argv(),
        vec![
            OsString::from("clone"),
            OsString::from("https://github.com/me/repo.git"),
        ]
    );
    assert_eq!(cmd.build_display(), "git clone https://github.com/me/repo.git");
}
