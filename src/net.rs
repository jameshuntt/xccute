// crates/shell_commands/src/net.rs

pub mod curl;
pub use curl::*;

pub mod ftp;
pub use ftp::*;

pub mod ip;
pub use ip::*;

pub mod nc;
pub use nc::*;

pub mod netstat;
pub use netstat::*;

pub mod ping;
pub use ping::*;

pub mod rsync;
pub use rsync::*;

pub mod scp;
pub use scp::*;

pub mod sftp;
pub use sftp::*;

pub mod ss;
pub use ss::*;

pub mod telnet;
pub use telnet::*;

pub mod wget;
pub use wget::*;
