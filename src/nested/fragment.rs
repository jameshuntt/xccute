use std::process::Output;

use crate::runner::error::CommandError;

/// One modular unit in a nested command.
/// Examples:
/// - program-global options (no segment)
/// - "remote" stage
/// - "add" stage (more specific, may override exit handling)
pub trait ShellFragment: Send + Sync {
    /// Segment to add to argv path (e.g. Some("remote")), or None for global stage.
    fn segment(&self) -> Option<&'static str> {
        None
    }

    /// Push args/flags contributed by this stage.
    fn push_args(&self, out: &mut Vec<String>);

    /// If this stage wants to define “ok exit codes”, return Some(slice).
    /// More-specific stages win (last stage in the chain that returns Some).
    fn ok_codes(&self) -> Option<&'static [i32]> {
        None
    }

    /// If this stage wants to override output handling, return Some(result).
    /// More-specific stages win (last stage in the chain that returns Some).
    fn handle_output(&self, _output: &Output, _built: &str) -> Option<Result<(), CommandError>> {
        None
    }
}
