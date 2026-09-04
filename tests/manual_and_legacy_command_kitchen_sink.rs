//! Kitchen-sink examples for manual and compatibility-layer command construction.
//!
//! This file shows the older/raw surfaces that still matter while the generated
//! composite command stack becomes the preferred API:
//!
//! - `RawCommand` shell string compatibility.
//! - handwritten `ShellCommand` implementations.
//! - argv-safe `to_command()` overrides.
//! - legacy `ShellSubCommand` + `ShellProgram` composition.
//! - existing hand-written builders like `MkdirBuilder`, `EchoBuilder`, and
//!   `GrepBuilder`.
//! - dry-run command-chain execution without spawning real processes.

use std::ffi::OsStr;
use std::process::Command;

use xccute::{
    EchoBuilder, GrepBuilder, MkdirBuilder, RawCommand, ShellCommand, ShellProgram,
    ShellSubCommand,
};
use xccute::runner::executor::{CaptureMode, CommandChainExecutor};

#[derive(Debug, Clone)]
struct ManualArgvCommand {
    program: &'static str,
    args: Vec<String>,
    display: String,
}

impl ManualArgvCommand {
    fn new(program: &'static str, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let args = args.into_iter().map(Into::into).collect::<Vec<_>>();
        let display = std::iter::once(program.to_string())
            .chain(args.iter().cloned())
            .collect::<Vec<_>>()
            .join(" ");

        Self {
            program,
            args,
            display,
        }
    }
}

impl ShellCommand for ManualArgvCommand {
    fn build(&self) -> String {
        self.display.clone()
    }

    fn to_command(&self) -> Command {
        let mut command = Command::new(self.program);
        command.args(&self.args);
        command
    }

    fn ok_codes(&self) -> &'static [i32] {
        &[0, 1]
    }
}

#[derive(Debug, Clone)]
struct LegacyGitRemoteAdd {
    name: String,
    url: String,
}

impl ShellSubCommand for LegacyGitRemoteAdd {
    fn segments(&self) -> &'static [&'static str] {
        &["remote", "add"]
    }

    fn args(&self) -> Vec<String> {
        vec![self.name.clone(), self.url.clone()]
    }
}

#[test]
fn raw_and_handwritten_shell_commands_show_the_low_level_surfaces() {
    let raw = RawCommand("printf 'hello from raw shell'".to_string());
    assert_eq!(raw.build(), "printf 'hello from raw shell'");

    let manual = ManualArgvCommand::new("printf", ["%s", "hello argv"]);
    assert_eq!(manual.build(), "printf %s hello argv");
    assert_eq!(manual.ok_codes(), &[0, 1]);

    let std_command = manual.to_command();
    assert_eq!(std_command.get_program(), OsStr::new("printf"));
    assert_eq!(
        std_command
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect::<Vec<_>>(),
        vec!["%s".to_string(), "hello argv".to_string()]
    );
}

#[test]
fn legacy_shell_subcommand_composes_program_segments_and_args() {
    let cmd = ShellProgram::new(
        "git",
        LegacyGitRemoteAdd {
            name: "origin".to_string(),
            url: "https://example.invalid/repo.git".to_string(),
        },
    );

    assert_eq!(
        cmd.build(),
        "git remote add origin https://example.invalid/repo.git"
    );

    let std_command = cmd.to_command();
    assert_eq!(std_command.get_program(), OsStr::new("git"));
    assert_eq!(
        std_command
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect::<Vec<_>>(),
        vec![
            "remote".to_string(),
            "add".to_string(),
            "origin".to_string(),
            "https://example.invalid/repo.git".to_string(),
        ]
    );
}

#[test]
fn existing_handwritten_builders_remain_readable_examples() {
    let mkdir = MkdirBuilder::new("target/demo")
        .parents()
        .verbose()
        .mode("755");
    assert_eq!(mkdir.build(), "mkdir -p -v -m 755 target/demo");

    let echo = EchoBuilder::new("hello")
        .quoted()
        .no_newline()
        .to_file("target/demo/out.txt")
        .append();
    assert_eq!(echo.build(), "echo -n \"hello\" >> target/demo/out.txt");

    let grep = GrepBuilder::new("NEEDLE")
        .ignore_case()
        .line_number()
        .word_regexp()
        .file("logs/app.log")
        .file("logs/worker.log");
    assert_eq!(
        grep.build(),
        "grep -i -n -w NEEDLE logs/app.log logs/worker.log"
    );
}

#[test]
fn dry_run_chain_can_hold_mixed_command_shapes_without_spawning() {
    let chain = CommandChainExecutor::new()
        .with_dry_run(true)
        .with_stop_on_error(true)
        .with_capture(CaptureMode::Output)
        .add_command(RawCommand("echo raw".to_string()))
        .add_command(ManualArgvCommand::new("echo", ["manual", "argv"]))
        .add_command(ShellProgram::new(
            "git",
            LegacyGitRemoteAdd {
                name: "origin".to_string(),
                url: "https://example.invalid/repo.git".to_string(),
            },
        ));

    assert_eq!(chain.commands.len(), 3);
    chain.run().expect("dry-run mode should not spawn commands");
}
