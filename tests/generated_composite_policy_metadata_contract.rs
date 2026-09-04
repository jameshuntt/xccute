use std::ffi::{OsStr, OsString};

use simple_impl_derive::{CompositeShell, CompositeSubCommand, SimpleSubCommand};
use xccute::{
    XccuteArrangedCommand,
    XccuteCommandSensitivity,
    XccutePolicyMetadata,
};

#[derive(CompositeShell, Debug, Clone, Copy, Default)]
#[shell(program = "systemctl")]
#[composite(command = "restart", method = "restart", ty = SystemctlRestart, init = "unit")]
struct Systemctl;

#[derive(SimpleSubCommand, Debug, Clone)]
#[subcommand(segment = "restart")]
#[xccute_policy(
    sensitive,
    requires_sudo,
    iam_scope = "node:command",
    path_role = "systemd_service_unit",
    dry_run_default = true
)]
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
#[xccute_policy(iam_scope = "git:remote", path_role = "repository_remote")]
#[composite(command = "add", method = "add", ty = GitRemoteAdd, init = "name,url")]
struct GitRemote;

#[derive(SimpleSubCommand, Debug, Clone)]
#[subcommand(segment = "add")]
#[xccute_policy(sensitive)]
struct GitRemoteAdd {
    #[builder(init_required, into)]
    #[arg(positional)]
    name: OsString,

    #[builder(init_required, into)]
    #[arg(positional)]
    url: OsString,
}

#[derive(CompositeShell, Debug, Clone, Copy, Default)]
#[shell(program = "pgrep")]
#[composite(command = "pattern", method = "pattern", ty = PgrepPattern, init = "pattern")]
struct Pgrep;

#[derive(SimpleSubCommand, Debug, Clone)]
#[subcommand(segment = "match")]
struct PgrepPattern {
    #[builder(init_required, into)]
    #[arg(positional)]
    pattern: OsString,
}

#[test]
fn simple_subcommand_policy_metadata_is_available_on_rooted_command() {
    let cmd = Systemctl::restart("fleet-worker.service");
    let arrangement = cmd.arrange();
    let policy = cmd.xccute_policy();

    assert_eq!(arrangement.program(), OsStr::new("systemctl"));
    let expected = vec![
        OsString::from("restart"),
        OsString::from("fleet-worker.service"),
    ];
    assert_eq!(arrangement.argv(), expected.as_slice());

    assert_eq!(policy.sensitivity(), XccuteCommandSensitivity::Sensitive);
    assert!(policy.is_sensitive());
    assert!(policy.requires_sudo());
    assert_eq!(policy.iam_scope(), Some("node:command"));
    let expected_roles = vec![String::from("systemd_service_unit")];
    assert_eq!(policy.path_roles(), expected_roles.as_slice());
    assert!(policy.dry_run_default());
}

#[test]
fn composite_surface_policy_metadata_merges_with_leaf_subcommand_policy() {
    let cmd = Git::remote().add("origin", "https://example.invalid/repo.git");
    let arrangement = cmd.arrange();
    let policy = cmd.xccute_policy();

    assert_eq!(arrangement.program(), OsStr::new("git"));
    let expected = vec![
        OsString::from("remote"),
        OsString::from("add"),
        OsString::from("origin"),
        OsString::from("https://example.invalid/repo.git"),
    ];
    assert_eq!(arrangement.argv(), expected.as_slice());

    assert!(policy.is_sensitive());
    assert!(!policy.requires_sudo());
    assert_eq!(policy.iam_scope(), Some("git:remote"));
    let expected_roles = vec![String::from("repository_remote")];
    assert_eq!(policy.path_roles(), expected_roles.as_slice());
    assert!(policy.dry_run_default());
}

#[test]
fn unannotated_subcommand_policy_defaults_to_unprivileged_dry_run_metadata() {
    let cmd = Pgrep::pattern("fleet");
    let arrangement = cmd.arrange();
    let policy = cmd.xccute_policy();

    assert_eq!(arrangement.program(), OsStr::new("pgrep"));
    let expected = vec![OsString::from("match"), OsString::from("fleet")];
    assert_eq!(arrangement.argv(), expected.as_slice());
    assert_eq!(policy.sensitivity(), XccuteCommandSensitivity::Normal);
    assert!(!policy.is_sensitive());
    assert!(!policy.requires_sudo());
    assert_eq!(policy.iam_scope(), None);
    assert!(policy.path_roles().is_empty());
    assert!(policy.dry_run_default());
}
