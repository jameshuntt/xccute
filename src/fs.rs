// crates/shell_commands/src/proc.rs

pub mod chmod;
pub use chmod::*;

pub mod cd;
pub use cd::*;

pub mod chown;
pub use chown::*;

pub mod cp;
pub use cp::*;

pub mod df;
pub use df::*;

pub mod du;
pub use du::*;

pub mod echo;
pub use echo::*;

pub mod file;
pub use file::*;

pub mod ls;
pub use ls::*;

pub mod mkdir;
pub use mkdir::*;

pub mod mv;
pub use mv::*;

pub mod nano;
pub use nano::*;

pub mod rm;
pub use rm::*;

pub mod rmdir;
pub use rmdir::*;

pub mod stat;
pub use stat::*;

pub mod touch;
pub use touch::*;
