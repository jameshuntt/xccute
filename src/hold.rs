//! A step that holds a resource while its lines run: the resource taken within
//! a timeout (tried again after a backoff while it is busy), the lines walked
//! while it is held, the rollback lines walked when one of them fails, and the
//! resource let go after, whether they succeeded or failed.
//!
//! A resource is [`Holdable`]. [`LockFile`] is the fleet's: a lock file opened
//! read-only and never created, an exclusive `flock` on it; no file means
//! nothing has taken it since its folder was made, and the lines run without
//! it.

use std::any::Any;
use std::fmt;
use std::fs::{File, TryLockError};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::line_chain::{LineChain, Walked};
use crate::runtime::{ExitDisposition, ExitStatusDecision, RuntimeExitStatus, RuntimeOperation};
use crate::timing::{HOLD_RETRY_BACKOFF, HOLD_TIMEOUT};

/// What one try at a resource found.
pub enum Taken {
    /// Taken: it is held until the guard is dropped.
    Held(Box<dyn Any + Send>),
    /// Someone else holds it; try again later.
    Busy,
    /// There is nothing to hold (a lock file that is not there): the lines run
    /// without it.
    Absent,
}

impl fmt::Debug for Taken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Taken::Held(_) => "Held",
            Taken::Busy => "Busy",
            Taken::Absent => "Absent",
        })
    }
}

/// A resource a [`Hold`] takes: a lock file, a lease, a slot.
pub trait Holdable: fmt::Debug + Send + Sync {
    /// What is held, as a line that says why names it (`/run/kit/anchor.lock`).
    fn name(&self) -> String;

    /// One try, never waiting: taken, busy, or nothing there to hold.
    fn try_take(&self) -> io::Result<Taken>;
}

/// A lock file, held by an exclusive `flock`: opened read-only and never
/// created, so a file that is not there is [`Taken::Absent`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockFile {
    path: PathBuf,
}

impl LockFile {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Holdable for LockFile {
    fn name(&self) -> String {
        self.path.display().to_string()
    }

    fn try_take(&self) -> io::Result<Taken> {
        let file = match File::open(&self.path) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Taken::Absent),
            other => other?,
        };
        match file.try_lock() {
            Ok(()) => Ok(Taken::Held(Box::new(file))),
            Err(TryLockError::WouldBlock) => Ok(Taken::Busy),
            Err(TryLockError::Error(e)) => Err(e),
        }
    }
}

/// How a hold went.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoldState {
    /// The resource was taken, and let go after the lines.
    Held,
    /// There was nothing to hold; the lines ran without it.
    Absent,
    /// It stayed busy for the whole timeout; no line ran.
    Busy { waited: Duration },
    /// Trying it failed (`why`); no line ran.
    Failed { why: String },
}

/// A hold walked: what happened to the resource, the lines walked while it was
/// held, and the rollback lines walked after a failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoldWalked {
    pub state: HoldState,
    pub walked: Vec<Walked>,
    pub rolled_back: Vec<Walked>,
}

impl HoldWalked {
    /// Whether the hold failed: the resource was never taken, or a line it held
    /// for stopped on a status it does not accept.
    pub fn failed(&self) -> bool {
        matches!(self.state, HoldState::Busy { .. } | HoldState::Failed { .. }) || failed(&self.walked)
    }
}

/// A wait as a line says it: whole seconds as `N s`, anything finer as `N ms`.
pub fn how_long(wait: Duration) -> String {
    if wait.subsec_nanos() == 0 {
        format!("{} s", wait.as_secs())
    } else {
        format!("{} ms", wait.as_millis())
    }
}

/// Whether a walk ended on a line that stopped on a status it does not accept.
fn failed(walked: &[Walked]) -> bool {
    walked.last().is_some_and(|w| w.decision.disposition == ExitDisposition::Stop && !w.decision.status.accepted)
}

/// A step that holds a resource while its lines run. The resource is tried
/// until it is taken or the timeout passes, `retry_backoff` apart; the lines
/// are walked while it is held; when one of them fails the rollback lines are
/// walked, still holding it; the resource is let go after, either way.
#[derive(Debug, Clone)]
pub struct Hold {
    pub id: String,
    resource: Arc<dyn Holdable>,
    timeout: Duration,
    retry_backoff: Duration,
    lines: LineChain,
    rollback: Option<LineChain>,
}

impl Hold {
    /// A hold named `id` on `resource` around `lines`, waiting
    /// [`HOLD_TIMEOUT`] and trying every [`HOLD_RETRY_BACKOFF`].
    pub fn new(id: impl Into<String>, resource: impl Holdable + 'static, lines: LineChain) -> Self {
        Self {
            id: id.into(),
            resource: Arc::new(resource),
            timeout: HOLD_TIMEOUT,
            retry_backoff: HOLD_RETRY_BACKOFF,
            lines,
            rollback: None,
        }
    }

    /// The longest the hold waits for its resource.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// How long it waits between two tries at a busy resource.
    pub fn retry_backoff(mut self, retry_backoff: Duration) -> Self {
        self.retry_backoff = retry_backoff;
        self
    }

    /// The lines walked, still holding the resource, when a line it holds for
    /// fails.
    pub fn rollback(mut self, rollback: LineChain) -> Self {
        self.rollback = Some(rollback);
        self
    }

    pub fn resource(&self) -> &dyn Holdable {
        self.resource.as_ref()
    }

    pub fn wait(&self) -> (Duration, Duration) {
        (self.timeout, self.retry_backoff)
    }

    pub fn lines(&self) -> &LineChain {
        &self.lines
    }

    pub fn rollback_lines(&self) -> Option<&LineChain> {
        self.rollback.as_ref()
    }

    /// The hold walked: the resource taken, `run` running each line (its exit
    /// status, `None` for a signal or a line that never started), the rollback
    /// after a failure, the resource let go.
    pub fn walk(&self, run: &mut dyn FnMut(&RuntimeOperation) -> Option<i32>) -> Result<HoldWalked, String> {
        let deadline = Instant::now() + self.timeout;
        let guard = loop {
            match self.resource.try_take() {
                Ok(Taken::Held(guard)) => break Some(guard),
                Ok(Taken::Absent) => break None,
                Ok(Taken::Busy) if Instant::now() < deadline => std::thread::sleep(self.retry_backoff),
                Ok(Taken::Busy) => return Ok(HoldWalked { state: HoldState::Busy { waited: self.timeout }, walked: Vec::new(), rolled_back: Vec::new() }),
                Err(e) => return Ok(HoldWalked { state: HoldState::Failed { why: format!("{}: {e}", self.resource.name()) }, walked: Vec::new(), rolled_back: Vec::new() }),
            }
        };
        let state = if guard.is_some() { HoldState::Held } else { HoldState::Absent };
        // the guard is held across the lines and the rollback, and dropped
        // after both, on every way out of here
        let walked = self.lines.walk_with(run);
        let walked = match walked {
            Ok(walked) => walked,
            Err(e) => {
                drop(guard);
                return Err(e);
            }
        };
        let rolled_back = match (&self.rollback, failed(&walked)) {
            (Some(rollback), true) => rollback.walk_with(run),
            _ => Ok(Vec::new()),
        };
        drop(guard);
        Ok(HoldWalked { state, walked, rolled_back: rolled_back? })
    }

    /// The hold as one walked step of a chain: its id, no status of its own, and
    /// a decision that goes on when it held and its lines succeeded and stops
    /// otherwise, its reason saying which.
    pub fn decision(&self, walked: &HoldWalked) -> ExitStatusDecision {
        let name = self.resource.name();
        let ok = !walked.failed();
        let reason = match &walked.state {
            HoldState::Busy { waited } => format!("{name} was held for {}: {} not run", how_long(*waited), self.id),
            HoldState::Failed { why } => format!("{why}: {} not run", self.id),
            state => {
                let holding = if *state == HoldState::Held { format!("holding {name}") } else { format!("no {name}: nothing to wait for") };
                match (ok, walked.rolled_back.is_empty()) {
                    (true, _) => format!("{} ran {holding}", self.id),
                    (false, true) => format!("{} failed {holding}", self.id),
                    (false, false) => format!("{} failed {holding}; rolled back", self.id),
                }
            }
        };
        ExitStatusDecision {
            status: RuntimeExitStatus::new(None, ok),
            disposition: if ok { ExitDisposition::Continue } else { ExitDisposition::Stop },
            reason,
        }
    }
}
