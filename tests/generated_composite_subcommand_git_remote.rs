use std::ffi::{OsStr, OsString};

use simple_impl_derive::{CompositeShell, CompositeSubCommand, SimpleSubCommand};
use xccute::CompositeShellCommand;

#[derive(CompositeShell, Debug, Clone, Copy, Default)]
#[shell(program = "git")]
#[composite(surface = "remote", method = "remote", ty = GitRemote)]
struct Git;

#[derive(CompositeSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "remote")]
#[composite(command = "add", method = "add", ty = GitRemoteAdd, init = "name,url")]
struct GitRemote;

#[derive(SimpleSubCommand, Debug, Clone)]
#[subcommand(segment = "add")]
struct GitRemoteAdd {
    #[builder(init_required, into)]
    #[arg(positional)]
    name: OsString,

    #[builder(init_required, into)]
    #[arg(positional)]
    url: OsString,
}

#[test]
fn generated_composite_subcommand_adds_nested_root_command_method() {
    let cmd = Git::remote().add("origin", "https://github.com/me/repo.git");

    assert_eq!(CompositeShellCommand::program(&cmd), OsStr::new("git"));
    assert_eq!(
        cmd.argv(),
        vec![
            OsString::from("remote"),
            OsString::from("add"),
            OsString::from("origin"),
            OsString::from("https://github.com/me/repo.git"),
        ]
    );
    assert_eq!(
        cmd.build_display(),
        "git remote add origin https://github.com/me/repo.git"
    );
}
