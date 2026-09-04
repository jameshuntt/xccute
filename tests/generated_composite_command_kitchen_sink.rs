//! Kitchen-sink examples for the modern argv-safe generated command surface.
//!
//! This file is intentionally example-heavy. It shows the patterns that are
//! currently available through the derive stack:
//!
//! - `SimpleSubCommand` for a leaf command segment.
//! - `CompositeShell` for a root executable namespace.
//! - registry-style `#[composite(...)]` root entries.
//! - field-style `#[composite(...)]` root entries.
//! - `CompositeSubCommand` for a nested command surface.
//! - required positional args, optional key/value args, boolean flags.
//! - generated setters on direct root commands.
//! - nested surface init-forwarding.
//! - validation, preview, approval, and the `ShellCommand` compatibility facade.
//!
//! Note: today's `CompositeSubCommand` wrapper forwards nested init args and argv
//! parts, but it does not yet re-expose the leaf command's optional setters.
//! These tests deliberately make that current boundary visible before Pass 54.

use std::ffi::{OsStr, OsString};

use simple_impl_derive::{CompositeShell, CompositeSubCommand, SimpleSubCommand};
use xccute::{
    CommandApproval, CommandPolicyError, CommandPolicyResult, CommandPreview,
    CommandPreviewPolicy, CommandValidationError, CommandValidationErrorKind,
    CommandValidationResult, CompositeShellCommand, ShellCommand, ValidatedCommand,
    ValidatedCompositeShellCommand,
};

// -----------------------------------------------------------------------------
// Registry-style root command + nested surface declarations
// -----------------------------------------------------------------------------

#[derive(CompositeShell, Debug, Clone, Copy, Default)]
#[shell(program = "orb")]
#[composite(command = "run", method = "run", ty = OrbRun, init = "target")]
#[composite(command = "status", method = "status", ty = OrbStatus)]
#[composite(surface = "project", method = "project", ty = OrbProject)]
struct Orb;

#[derive(SimpleSubCommand, Debug, Clone)]
#[subcommand(segment = "run")]
#[validate(with = "validate_run_target")]
struct OrbRun {
    // Required init args are forwarded by the generated root method:
    // `Orb::run("worker-a")`.
    #[builder(init_required, into)]
    #[arg(positional)]
    target: OsString,

    // Optional key/value args generate a setter:
    // `.profile("prod")` => `--profile prod`.
    #[builder(method = "profile", into)]
    #[arg(kv = "--profile")]
    profile: Option<OsString>,

    // Boolean flags generate fluent no-arg setters:
    // `.dry_run()` => `--dry-run`.
    #[arg(flag = "--dry-run")]
    dry_run: bool,

    #[arg(flag = "--json")]
    json: bool,
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "status")]
#[allow(dead_code)]
struct OrbStatus {
    #[arg(flag = "--short")]
    short: bool,

    #[builder(method = "scope", into)]
    #[arg(kv = "--scope")]
    scope: Option<OsString>,
}

#[derive(CompositeSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "project")]
#[composite(command = "create", method = "create", ty = OrbProjectCreate, init = "name")]
#[composite(command = "set-owner", method = "set_owner", ty = OrbProjectSetOwner, init = "name,owner")]
struct OrbProject;

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "create")]
struct OrbProjectCreate {
    #[builder(init_required, into)]
    #[arg(positional)]
    name: OsString,

    #[builder(method = "template", into)]
    #[arg(kv = "--template")]
    template: Option<OsString>,

    #[arg(flag = "--private")]
    private: bool,
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "set-owner")]
struct OrbProjectSetOwner {
    #[builder(init_required, into)]
    #[arg(positional)]
    name: OsString,

    #[builder(init_required, into)]
    #[arg(positional)]
    owner: OsString,

    #[arg(flag = "--notify")]
    notify: bool,
}

fn validate_run_target(cmd: &OrbRun) -> CommandValidationResult {
    if cmd.target.to_string_lossy().trim().is_empty() {
        return Err(CommandValidationError::runtime_preflight("run target cannot be empty")
            .with_field("target")
            .with_rule("validate_with"));
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Field-style root + field-style nested surface declarations
// -----------------------------------------------------------------------------

#[derive(CompositeShell, Debug, Clone, Default)]
#[shell(program = "orbctl")]
struct OrbCtl {
    // Field-style command declarations use the field name as the default segment
    // and method. `init = "id"` forwards to `OrbInspect::new(id)`.
    #[composite(command, init = "id")]
    inspect: OrbInspect,

    // Field-style surface declaration generates `OrbCtl::workspace()`.
    #[composite(surface)]
    workspace: OrbWorkspace,
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "inspect")]
struct OrbInspect {
    #[builder(init_required, into)]
    #[arg(positional)]
    id: OsString,

    #[arg(flag = "--verbose")]
    verbose: bool,
}

#[derive(CompositeSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "workspace")]
struct OrbWorkspace {
    #[composite(command, init = "name")]
    open: OrbWorkspaceOpen,
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "open")]
struct OrbWorkspaceOpen {
    #[builder(init_required, into)]
    #[arg(positional)]
    name: OsString,

    #[builder(method = "mode", into)]
    #[arg(kv = "--mode")]
    mode: Option<OsString>,
}

// -----------------------------------------------------------------------------
// Validation-only command that demonstrates structural rule attrs.
// -----------------------------------------------------------------------------

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "render")]
#[one_of("json", "yaml")]
struct OrbRenderFormat {
    #[arg(flag = "--json")]
    json: bool,

    #[arg(flag = "--yaml")]
    yaml: bool,
}

// -----------------------------------------------------------------------------
// Policy hook used by the preview/approval test.
// -----------------------------------------------------------------------------

struct AllowOrbRunDryRunPolicy;

impl CommandPreviewPolicy for AllowOrbRunDryRunPolicy {
    fn approve(&self, preview: &CommandPreview) -> CommandPolicyResult {
        if preview.program() != OsStr::new("orb") {
            return Err(CommandPolicyError::new(
                "orb_dry_run_policy",
                "only orb commands are approved",
            ));
        }

        let expected = vec![
            OsString::from("run"),
            OsString::from("worker-a"),
            OsString::from("--profile"),
            OsString::from("prod"),
            OsString::from("--dry-run"),
        ];

        if preview.argv() != expected.as_slice() {
            return Err(CommandPolicyError::new(
                "orb_dry_run_policy",
                "only orb run worker-a --profile prod --dry-run is approved",
            ));
        }

        Ok(CommandApproval::new("orb_dry_run_policy", "operator")
            .with_reason("argv-safe preview matched the dry-run policy"))
    }
}

#[test]
fn registry_style_root_command_uses_init_optional_kv_and_flags() {
    let cmd = Orb::run("worker-a").profile("prod").dry_run();

    assert_eq!(CompositeShellCommand::program(&cmd), OsStr::new("orb"));
    assert_eq!(
        cmd.argv(),
        vec![
            OsString::from("run"),
            OsString::from("worker-a"),
            OsString::from("--profile"),
            OsString::from("prod"),
            OsString::from("--dry-run"),
        ]
    );
    assert_eq!(cmd.build_display(), "orb run worker-a --profile prod --dry-run");

    // The generated composite command also flows through the older ShellCommand
    // facade, but still executes argv-safely because the facade delegates to the
    // CompositeShellCommand implementation.
    assert_eq!(ShellCommand::build(&cmd), "orb run worker-a --profile prod --dry-run");
}

#[test]
fn registry_style_nested_surface_builds_multi_segment_command() {
    let create = Orb::project().create("control-plane");

    assert_eq!(
        create.argv(),
        vec![
            OsString::from("project"),
            OsString::from("create"),
            OsString::from("control-plane"),
        ]
    );
    assert_eq!(create.build_display(), "orb project create control-plane");

    let set_owner = Orb::project().set_owner("control-plane", "james");

    assert_eq!(
        set_owner.argv(),
        vec![
            OsString::from("project"),
            OsString::from("set-owner"),
            OsString::from("control-plane"),
            OsString::from("james"),
        ]
    );
    assert_eq!(
        set_owner.build_display(),
        "orb project set-owner control-plane james"
    );

    // Current boundary: the nested wrapper emits the surface segment, nested
    // command segment, and init args. It does not yet re-expose optional setters
    // from the leaf command such as `.template(...)`, `.private()`, or `.notify()`.
    // That is useful to see before the interface-pattern work, because it is a
    // real API shape decision rather than a training-row concern.
}

#[test]
fn field_style_root_and_surface_generate_the_same_kind_of_api() {
    let inspect = OrbCtl::inspect("node-17").verbose();

    assert_eq!(CompositeShellCommand::program(&inspect), OsStr::new("orbctl"));
    assert_eq!(
        inspect.argv(),
        vec![
            OsString::from("inspect"),
            OsString::from("node-17"),
            OsString::from("--verbose"),
        ]
    );
    assert_eq!(inspect.build_display(), "orbctl inspect node-17 --verbose");

    let workspace = OrbCtl::workspace().open("local");

    assert_eq!(
        workspace.argv(),
        vec![
            OsString::from("workspace"),
            OsString::from("open"),
            OsString::from("local"),
        ]
    );
    assert_eq!(workspace.build_display(), "orbctl workspace open local");
}

#[test]
fn generated_validation_runs_before_preview_or_approval() {
    let err = Orb::run("")
        .dry_run()
        .validated_preview()
        .unwrap_err();

    assert_eq!(err.kind(), &CommandValidationErrorKind::RuntimePreflight);
    assert_eq!(err.field(), Some("target"));
    assert_eq!(err.rule(), Some("validate_with"));

    let no_format = OrbRenderFormat::default().rooted::<Orb>();
    let err = no_format.validate().unwrap_err();

    assert_eq!(err.kind(), &CommandValidationErrorKind::Structural);
    assert_eq!(err.rule(), Some("one_of"));

    OrbRenderFormat::default()
        .rooted::<Orb>()
        .json()
        .validate()
        .unwrap();
}

#[test]
fn generated_preview_and_policy_receipt_are_no_side_effect_boundaries() {
    let cmd = Orb::run("worker-a").profile("prod").dry_run();
    let preview = cmd.validated_preview().unwrap();

    assert_eq!(preview.program(), OsStr::new("orb"));
    assert_eq!(preview.display(), "orb run worker-a --profile prod --dry-run");

    let receipt = cmd.approved_preview(&AllowOrbRunDryRunPolicy).unwrap();

    assert_eq!(receipt.preview().display(), preview.display());
    assert_eq!(receipt.approval().policy(), "orb_dry_run_policy");
    assert_eq!(receipt.approval().actor(), "operator");
    assert_eq!(
        receipt.approval().reason(),
        Some("argv-safe preview matched the dry-run policy")
    );
}
