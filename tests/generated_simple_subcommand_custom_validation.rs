use std::ffi::{OsStr, OsString};

use simple_impl_derive::SimpleSubCommand;
use xccute::{
    CommandValidationError,
    CommandValidationErrorKind,
    CommandValidationResult,
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

#[derive(SimpleSubCommand, Debug, Clone)]
#[subcommand(segment = "clone")]
#[validate(with = "validate_clone_url_preflight")]
struct GitCloneChecked {
    #[builder(init_required, into)]
    #[arg(positional)]
    url: OsString,
}

#[derive(SimpleSubCommand, Debug, Clone)]
#[subcommand(segment = "push")]
struct GitPushChecked {
    #[builder(init_required, into)]
    #[arg(positional)]
    #[validate(with = "validate_push_target_ack")]
    target: OsString,
}

fn validate_clone_url_preflight(cmd: &GitCloneChecked) -> CommandValidationResult {
    let url = cmd.url.to_string_lossy();

    if url.trim().is_empty() {
        return Err(CommandValidationError::runtime_preflight("clone url cannot be empty")
            .with_field("url")
            .with_rule("validate_with"));
    }

    if url.contains(" ") {
        return Err(CommandValidationError::runtime_preflight("clone url cannot contain spaces")
            .with_field("url")
            .with_rule("validate_with"));
    }

    Ok(())
}

fn validate_push_target_ack(cmd: &GitPushChecked) -> CommandValidationResult {
    let target = cmd.target.to_string_lossy();

    if target == "production" {
        return Err(CommandValidationError::custom(
            "production push requires a higher-level operator acknowledgment",
        )
        .with_field("target")
        .with_rule("validate_with"));
    }

    Ok(())
}

#[test]
fn generated_simple_subcommand_calls_command_level_preflight_hook() {
    let err = GitCloneChecked::new("")
        .rooted::<Git>()
        .validate()
        .unwrap_err();

    assert_eq!(err.kind(), &CommandValidationErrorKind::RuntimePreflight);
    assert_eq!(err.field(), Some("url"));
    assert_eq!(err.rule(), Some("validate_with"));

    GitCloneChecked::new("https://github.com/me/repo.git")
        .rooted::<Git>()
        .validate()
        .unwrap();
}

#[test]
fn generated_simple_subcommand_calls_field_level_custom_hook_against_command_contract() {
    let err = GitPushChecked::new("production")
        .rooted::<Git>()
        .validate()
        .unwrap_err();

    assert_eq!(err.kind(), &CommandValidationErrorKind::Custom);
    assert_eq!(err.field(), Some("target"));
    assert_eq!(err.rule(), Some("validate_with"));

    GitPushChecked::new("origin")
        .rooted::<Git>()
        .validate()
        .unwrap();
}
