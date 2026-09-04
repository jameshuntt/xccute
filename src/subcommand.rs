use std::process::{Command, Output};

use crate::runner::error::CommandError;
use crate::ShellCommand;

/// A typed subcommand that can represent multi-part paths like:
///   ["remote", "add"]
pub trait ShellSubCommand {
    /// Subcommand path segments (supports git-style “modifier modifier”).
    fn segments(&self) -> &'static [&'static str];

    /// Flags/args for the subcommand.
    fn args(&self) -> Vec<String>;

    /// Per-subcommand policy (default: only 0).
    fn ok_codes(&self) -> &'static [i32] {
        &[0]
    }

    fn handle_output(&self, output: &Output, full_cmd_display: &str) -> Result<(), CommandError> {
        let ok = match output.status.code() {
            Some(c) => self.ok_codes().contains(&c),
            None => output.status.success(),
        };

        if ok {
            Ok(())
        } else {
            Err(CommandError::from_output(full_cmd_display.to_string(), output))
        }
    }
}

/// Binds a base program name (e.g. "git") with a typed subcommand.
#[derive(Debug, Clone)]
pub struct ShellProgram<S> {
    pub program: &'static str,
    pub sub: S,
}

impl<S> ShellProgram<S> {
    pub fn new(program: &'static str, sub: S) -> Self {
        Self { program, sub }
    }
}

impl<S: ShellSubCommand> ShellCommand for ShellProgram<S> {
    fn build(&self) -> String {
        let mut parts = vec![self.program.to_string()];
        parts.extend(self.sub.segments().iter().map(|s| (*s).to_string()));
        parts.extend(self.sub.args());
        parts.join(" ")
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(self.program);
        cmd.args(self.sub.segments());
        cmd.args(self.sub.args());
        cmd
    }

    fn ok_codes(&self) -> &'static [i32] {
        self.sub.ok_codes()
    }

    fn handle_output(&self, output: &Output) -> Result<(), CommandError> {
        self.sub.handle_output(output, &self.build())
    }
}
