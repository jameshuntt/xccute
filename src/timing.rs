//! Every duration xccute waits by, declared once. A step that waits names its
//! wait here, and a caller that needs another one passes it to the step
//! (`Hold::timeout`, `Hold::retry_backoff`); nothing else in xccute holds a
//! number of seconds.
//!
//! James's time kit is in the kit and not a crate yet. When it is one, xccute
//! takes these durations from it, and this table names its values.

use std::time::Duration;

/// The longest a [`Hold`](crate::hold::Hold) waits for its resource when it
/// names no timeout of its own.
pub const HOLD_TIMEOUT: Duration = Duration::from_secs(60);

/// How long a [`Hold`](crate::hold::Hold) waits between two tries at a
/// resource that is busy (the fleet's lock file is tried every 100 ms).
pub const HOLD_RETRY_BACKOFF: Duration = Duration::from_millis(100);

/// Every duration above, by name, in the order declared: what a dry run or a
/// receipt can show for the waits a chain used.
pub const TIMING: &[(&str, Duration)] = &[("hold_timeout", HOLD_TIMEOUT), ("hold_retry_backoff", HOLD_RETRY_BACKOFF)];
