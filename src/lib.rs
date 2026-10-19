#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

/// The traits a generated argv command implements.
pub use xccute_contract as contract;
/// Execution: `ShellCommand`, `RawCommand`, `CommandSpec`, the runner, the seams.
pub use xccute_runtime as runtime;
/// The command builders, by family.
pub use xccute_commands as commands;
/// The decision vocabulary: questions, observation tools, paths, runbooks.
pub use xccute_decisions as decisions;

pub use xccute_commands::*;
pub use xccute_runtime::*;

pub mod command_chain;
pub mod composite;
pub mod hold;
pub mod line_chain;
pub mod nested;
pub mod preset;
pub mod script_config;
pub mod shell_scripts;
pub mod snapshot;
pub mod subcommand;
pub mod timing;
pub mod toml_loader;

pub use command_chain::{CommandChain, CommandStep};
pub use composite::*;
pub use hold::{Hold, HoldState, HoldWalked, Holdable, LockFile, Taken};
pub use line_chain::{run_line, LineChain, LineStep, Walked};
pub use nested::{ConfigError, DynStageFactory, NestedCommand, NestedRegistry, ShellFragment, StageDecl, StageFactory};
pub use preset::*;
pub use script_config::*;
pub use shell_scripts::ScriptBuilder;
pub use snapshot::CommandChainSnapshot;
pub use subcommand::{ShellProgram, ShellSubCommand};
pub use toml_loader::from_config;
