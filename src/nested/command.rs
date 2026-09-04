use std::process::{Command, Output};

use crate::ShellCommand;
use crate::nested::fragment::ShellFragment;
use crate::runner::error::CommandError;

pub struct NestedCommand {
    program: String,
    stages: Vec<Box<dyn ShellFragment>>,
}

impl NestedCommand {
    pub fn new(program: impl Into<String>) -> Self {
        Self { program: program.into(), stages: Vec::new() }
    }

    pub fn push_stage(mut self, stage: Box<dyn ShellFragment>) -> Self {
        self.stages.push(stage);
        self
    }

    fn effective_ok_codes(&self) -> &'static [i32] {
        for s in self.stages.iter().rev() {
            if let Some(codes) = s.ok_codes() {
                return codes;
            }
        }
        &[0]
    }

    fn maybe_handle_output(&self, output: &Output, built: &str) -> Option<Result<(), CommandError>> {
        for s in self.stages.iter().rev() {
            if let Some(res) = s.handle_output(output, built) {
                return Some(res);
            }
        }
        None
    }
}

impl ShellCommand for NestedCommand {
    fn build(&self) -> String {
        let mut parts = vec![self.program.clone()];

        for s in &self.stages {
            if let Some(seg) = s.segment() {
                parts.push(seg.to_string());
            }
        }

        let mut args = Vec::new();
        for s in &self.stages {
            s.push_args(&mut args);
        }

        parts.extend(args);
        parts.join(" ")
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.program);

        for s in &self.stages {
            if let Some(seg) = s.segment() {
                cmd.arg(seg);
            }
        }

        let mut args = Vec::new();
        for s in &self.stages {
            s.push_args(&mut args);
        }
        cmd.args(args);

        cmd
    }

    fn ok_codes(&self) -> &'static [i32] {
        self.effective_ok_codes()
    }

    fn handle_output(&self, output: &Output) -> Result<(), CommandError> {
        let built = self.build();

        if let Some(res) = self.maybe_handle_output(output, &built) {
            return res;
        }

        // default: ok-codes check
        let ok = match output.status.code() {
            Some(c) => self.ok_codes().contains(&c),
            None => output.status.success(),
        };

        if ok {
            Ok(())
        } else {
            Err(CommandError::from_output(built, output))
        }
    }
}
