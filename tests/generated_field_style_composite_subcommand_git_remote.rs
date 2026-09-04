use std::ffi::{OsStr, OsString};

use simple_impl_derive::{CompositeShell, CompositeSubCommand, SimpleSubCommand};
use xccute::CompositeShellCommand;

#[derive(CompositeShell, Debug, Clone, Default)]
#[shell(program = "git")]
struct Git {
    #[composite(surface)]
    remote: GitRemote,
}

#[derive(CompositeSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "remote")]
struct GitRemote {
    #[composite(command, init = "name,url")]
    add: GitRemoteAdd,
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
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
fn field_style_composite_subcommand_generates_same_nested_api() {
    let cmd = Git::remote().add("origin", "https://example.invalid/repo.git");

    assert_eq!(CompositeShellCommand::program(&cmd), OsStr::new("git"));
    assert_eq!(
        cmd.argv(),
        vec![
            OsString::from("remote"),
            OsString::from("add"),
            OsString::from("origin"),
            OsString::from("https://example.invalid/repo.git"),
        ]
    );
    assert_eq!(
        cmd.build_display(),
        "git remote add origin https://example.invalid/repo.git"
    );
}
