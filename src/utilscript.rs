// crates/shell_commands/src/utilscript.rs

pub mod awk;
pub use awk::*;

pub mod cut;
pub use cut::*;

pub mod grep;
pub use grep::*;

pub mod head;
pub use head::*;

pub mod sed;
pub use sed::*;

pub mod sort;
pub use sort::*;

pub mod tail;
pub use tail::*;

pub mod tee;
pub use tee::*;

pub mod uniq;
pub use uniq::*;

pub mod wc;
pub use wc::*;

pub mod xargs;
pub use xargs::*;
