use std::ffi::{OsStr, OsString};

use simple_impl_derive::{CompositeShell, SimpleSubCommand};
use xccute::{
    CommandApproval, CommandPolicyError, CommandPolicyResult, CommandPreview,
    CommandPreviewPolicy, ValidatedCompositeShellCommand,
};

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

struct AllowGitStatusPolicy;

impl CommandPreviewPolicy for AllowGitStatusPolicy {
    fn approve(&self, preview: &CommandPreview) -> CommandPolicyResult {
        if preview.program() != OsStr::new("git") {
            return Err(CommandPolicyError::new(
                "git_status_policy",
                "only git commands are approved",
            ));
        }

        let expected = vec![OsString::from("status"), OsString::from("--short")];
        if preview.argv() != expected.as_slice() {
            return Err(CommandPolicyError::new(
                "git_status_policy",
                "only git status --short is approved",
            ));
        }

        Ok(CommandApproval::new("git_status_policy", "operator").with_reason("preview matched"))
    }
}

#[test]
fn generated_command_can_create_approved_no_side_effect_preview() {
    let cmd = Git::status().short();
    let receipt = cmd
        .approved_preview(&AllowGitStatusPolicy)
        .expect("generated command should validate and pass policy approval");

    assert_eq!(receipt.preview().program(), OsStr::new("git"));
    assert_eq!(
        receipt.preview().argv(),
        &[OsString::from("status"), OsString::from("--short")]
    );
    assert_eq!(receipt.preview().display(), "git status --short");
    assert_eq!(receipt.approval().policy(), "git_status_policy");
    assert_eq!(receipt.approval().actor(), "operator");
    assert_eq!(receipt.approval().reason(), Some("preview matched"));
}
