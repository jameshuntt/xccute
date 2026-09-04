//! Staged pipelines: a program, its path, and fragments declared in TOML.

pub mod command;
pub mod fragment;
pub mod registry;

pub use command::*;
pub use fragment::*;
pub use registry::*;
