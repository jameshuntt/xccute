use std::ffi::{OsStr, OsString};

use simple_impl_derive::{CompositeShell, SimpleSubCommand};
use xccute::{CompositeShellCommand, ValidatedCompositeShellCommand};

#[derive(CompositeShell, Debug, Clone, Copy, Default)]
#[shell(program = "git")]
#[composite(command = "status", method = "status", ty = GitStatus)]
struct Git;

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "status")]
struct GitStatus {
    #[arg(flag = "--short")]
    short: bool,
}

#[test]
fn generated_command_can_create_validated_no_side_effect_preview() {
    let cmd = Git::status().short();
    let preview = cmd
        .validated_preview()
        .expect("default validation should allow preview");

    assert_eq!(preview.program(), OsStr::new("git"));
    assert_eq!(
        preview.argv(),
        &[OsString::from("status"), OsString::from("--short")]
    );
    assert_eq!(preview.display(), "git status --short");

    assert_eq!(cmd.argv(), preview.argv());
}
