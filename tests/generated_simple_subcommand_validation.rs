use std::ffi::OsStr;

use simple_impl_derive::SimpleSubCommand;
use xccute::{
    CommandValidationErrorKind,
    CompositeShellRoot,
    ValidatedCommand,
};

#[derive(Debug, Clone, Copy, Default)]
struct Git;

impl CompositeShellRoot for Git {
    fn program() -> &'static OsStr {
        OsStr::new("git")
    }
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "branch")]
struct GitBranchDelete {
    #[arg(flag = "-d")]
    #[invalid_without("force")]
    delete: bool,

    #[arg(flag = "-f")]
    #[only_pair_with("delete")]
    force: bool,
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "status")]
struct GitStatusFormat {
    #[arg(flag = "--json")]
    #[conflicts_with("yaml")]
    json: bool,

    #[arg(flag = "--yaml")]
    yaml: bool,
}

#[test]
fn generated_simple_subcommand_blocks_invalid_without_rule() {
    let cmd = GitBranchDelete::default().rooted::<Git>().delete();
    let err = cmd.validate().unwrap_err();

    assert_eq!(err.kind(), &CommandValidationErrorKind::Structural);
    assert_eq!(err.field(), Some("delete"));
    assert_eq!(err.rule(), Some("invalid_without"));
}

#[test]
fn generated_simple_subcommand_blocks_only_pair_with_rule() {
    let cmd = GitBranchDelete::default().rooted::<Git>().force();
    let err = cmd.validate().unwrap_err();

    assert_eq!(err.kind(), &CommandValidationErrorKind::Structural);
    assert_eq!(err.field(), Some("force"));
    assert_eq!(err.rule(), Some("only_pair_with"));
}

#[test]
fn generated_simple_subcommand_accepts_acknowledged_pair() {
    let cmd = GitBranchDelete::default()
        .rooted::<Git>()
        .delete()
        .force();

    cmd.validate().unwrap();
}

#[test]
fn generated_simple_subcommand_blocks_conflicts_with_rule() {
    let cmd = GitStatusFormat::default()
        .rooted::<Git>()
        .json()
        .yaml();
    let err = cmd.validate().unwrap_err();

    assert_eq!(err.kind(), &CommandValidationErrorKind::Structural);
    assert_eq!(err.field(), Some("json"));
    assert_eq!(err.rule(), Some("conflicts_with"));
}
