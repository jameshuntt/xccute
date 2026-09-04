use std::ffi::{OsStr, OsString};

use simple_impl_derive::{CompositeShell, CompositeSubCommand, SimpleSubCommand};
use xccute::{CompositeShellCommand, XccuteArrangedCommand, XccuteCommandArrangement};

#[derive(CompositeShell, Debug, Clone, Copy, Default)]
#[shell(program = "systemctl")]
#[composite(command = "restart", method = "restart", ty = SystemctlRestart, init = "unit")]
struct Systemctl;

#[derive(SimpleSubCommand, Debug, Clone)]
#[subcommand(segment = "restart")]
struct SystemctlRestart {
    #[builder(init_required, into)]
    #[arg(positional)]
    unit: OsString,
}

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
fn direct_composite_subcommand_arranges_program_argv_and_display_without_shellcommand() {
    let cmd = Systemctl::restart("fleet-worker.service");
    let arrangement = cmd.arrange();

    assert_eq!(arrangement.program(), OsStr::new("systemctl"));
    let expected = vec![
        OsString::from("restart"),
        OsString::from("fleet-worker.service"),
    ];
    assert_eq!(arrangement.argv(), expected.as_slice());
    assert_eq!(
        arrangement.display(),
        "systemctl restart fleet-worker.service"
    );

    // The arrangement is the same trusted program + argv material as the
    // CompositeShellCommand preview. It does not depend on ShellCommand::build().
    assert_eq!(arrangement.preview(), &cmd.preview());
}

#[test]
fn nested_composite_surface_arranges_full_root_surface_leaf_argv() {
    let cmd = Git::remote().add("origin", "https://example.invalid/repo.git");
    let arrangement = cmd.arrange();

    assert_eq!(arrangement.program(), OsStr::new("git"));
    let expected = vec![
        OsString::from("remote"),
        OsString::from("add"),
        OsString::from("origin"),
        OsString::from("https://example.invalid/repo.git"),
    ];
    assert_eq!(arrangement.argv(), expected.as_slice());
    assert_eq!(
        arrangement.display(),
        "git remote add origin https://example.invalid/repo.git"
    );
}

#[test]
fn arrangement_can_be_rebuilt_from_preview_for_bridge_inputs() {
    let cmd = Git::remote().add("origin", "https://example.invalid/repo.git");
    let preview = cmd.preview();
    let arrangement = XccuteCommandArrangement::from_preview(preview.clone());

    assert_eq!(arrangement.preview(), &preview);
    assert_eq!(arrangement.program(), OsStr::new("git"));
    assert_eq!(arrangement.display(), preview.display());
}
