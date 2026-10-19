//! The hold step: a lock file taken within its timeout, its lines walked while
//! it is held, the rollback after a failure, the lock let go either way; and
//! the hold as a step of a chain. The lock is the fleet's (RestartHoldingLock):
//! opened read-only, never created, an exclusive flock.

use std::fs::File;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use xccute::contract::CommandPreview;
use xccute::timing::{HOLD_RETRY_BACKOFF, HOLD_TIMEOUT, TIMING};
use xccute::{ExitDisposition, ExitStatusPolicy, ExitStatusRule, Hold, HoldState, Holdable, LineChain, LockFile, Taken};

fn line(words: &[&str]) -> CommandPreview {
    CommandPreview::new(words[0].into(), words[1..].iter().map(Into::into).collect())
}

/// A fresh lock file in a folder of its own (never removed: each test names its own).
fn lock(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("xccute-hold-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("anchor.lock");
    File::create(&path).unwrap();
    path
}

/// The lock held by another holder (its own open file, as another process holds it).
fn taken_elsewhere(path: &PathBuf) -> File {
    let file = File::open(path).unwrap();
    file.try_lock().unwrap();
    file
}

fn restart(lock: &PathBuf) -> Hold {
    let lines = LineChain::new("restart").then("systemctl", &line(&["systemctl", "restart", "relayer.service"]), ExitStatusPolicy::new());
    Hold::new("restart", LockFile::new(lock), lines)
}

#[test]
fn the_timing_table_is_the_hold_s_default() {
    let hold = restart(&lock("defaults"));
    assert_eq!(hold.wait(), (HOLD_TIMEOUT, HOLD_RETRY_BACKOFF));
    assert_eq!(TIMING, [("hold_timeout", Duration::from_secs(60)), ("hold_retry_backoff", Duration::from_millis(100))]);
}

#[test]
fn a_lock_that_is_free_is_held_while_the_lines_run_and_let_go_after() {
    let path = lock("free");
    let mut held_while_running = false;
    let walked = restart(&path)
        .walk(&mut |_| {
            held_while_running = matches!(LockFile::new(&path).try_take().unwrap(), Taken::Busy);
            Some(0)
        })
        .unwrap();
    assert!(held_while_running, "the lock is held while the line runs");
    assert_eq!(walked.state, HoldState::Held);
    assert!(!walked.failed());
    assert!(matches!(LockFile::new(&path).try_take().unwrap(), Taken::Held(_)), "and let go after");
}

#[test]
fn no_lock_file_means_nothing_to_wait_for() {
    let path = std::env::temp_dir().join(format!("xccute-hold-{}-none/anchor.lock", std::process::id()));
    let hold = restart(&path);
    let walked = hold.walk(&mut |_| Some(0)).unwrap();
    assert_eq!(walked.state, HoldState::Absent);
    assert_eq!(walked.walked.len(), 1);
    assert!(!path.exists(), "the lock is never created");
    assert_eq!(hold.decision(&walked).reason, format!("restart ran no {}: nothing to wait for", path.display()));
}

#[test]
fn a_lock_held_past_the_timeout_runs_nothing() {
    let path = lock("busy");
    let _other = taken_elsewhere(&path);
    let hold = restart(&path).timeout(Duration::from_millis(300)).retry_backoff(Duration::from_millis(50));
    let started = Instant::now();
    let mut ran = 0;
    let walked = hold.walk(&mut |_| { ran += 1; Some(0) }).unwrap();
    assert!(started.elapsed() >= Duration::from_millis(300));
    assert_eq!(walked.state, HoldState::Busy { waited: Duration::from_millis(300) });
    assert_eq!(ran, 0);
    assert!(walked.failed());
    let decision = hold.decision(&walked);
    assert_eq!(decision.disposition, ExitDisposition::Stop);
    assert_eq!(decision.reason, format!("{} was held for 300 ms: restart not run", path.display()));
}

#[test]
fn a_lock_let_go_during_the_wait_is_taken() {
    let path = lock("released");
    let other = taken_elsewhere(&path);
    let holder = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(200));
        drop(other);
    });
    let walked = restart(&path).timeout(Duration::from_secs(5)).retry_backoff(Duration::from_millis(20)).walk(&mut |_| Some(0)).unwrap();
    holder.join().unwrap();
    assert_eq!(walked.state, HoldState::Held);
    assert_eq!(walked.walked.len(), 1);
}

#[test]
fn a_failed_line_is_rolled_back_still_holding_and_the_lock_is_let_go() {
    let path = lock("rollback");
    let lines = LineChain::new("swap")
        .then("stop", &line(&["systemctl", "stop", "a.service"]), ExitStatusPolicy::new())
        .then("install", &line(&["install", "new", "/usr/bin/a"]), ExitStatusPolicy::new())
        .then("start", &line(&["systemctl", "start", "a.service"]), ExitStatusPolicy::new());
    let rollback = LineChain::new("undo")
        .then("restore", &line(&["install", "old", "/usr/bin/a"]), ExitStatusPolicy::new())
        .then("start", &line(&["systemctl", "start", "a.service"]), ExitStatusPolicy::new());
    let hold = Hold::new("swap", LockFile::new(&path), lines).rollback(rollback);
    let mut held_in_rollback = false;
    let walked = hold
        .walk(&mut |op| {
            if op.argv.first().is_some_and(|w| w == "old") {
                held_in_rollback = matches!(LockFile::new(&path).try_take().unwrap(), Taken::Busy);
            }
            Some(if op.argv.first().is_some_and(|w| w == "new") { 1 } else { 0 })
        })
        .unwrap();
    assert_eq!(walked.walked.iter().map(|w| w.id.as_str()).collect::<Vec<_>>(), ["stop", "install"]);
    assert_eq!(walked.rolled_back.iter().map(|w| w.id.as_str()).collect::<Vec<_>>(), ["restore", "start"]);
    assert!(held_in_rollback, "the rollback runs holding the lock");
    assert!(walked.failed());
    assert_eq!(hold.decision(&walked).reason, format!("swap failed holding {}; rolled back", path.display()));
    assert!(matches!(LockFile::new(&path).try_take().unwrap(), Taken::Held(_)), "let go after a failure too");

    // a success walks no rollback
    let walked = hold.walk(&mut |_| Some(0)).unwrap();
    assert!(walked.rolled_back.is_empty() && !walked.failed());
}

#[test]
fn a_hold_is_a_step_of_a_chain() {
    let path = lock("chain");
    let chain = LineChain::new("deploy")
        .then("check", &line(&["test", "-f", "/etc/a"]), ExitStatusPolicy::new())
        .hold(restart(&path))
        .then("after", &line(&["systemctl", "is-active", "relayer.service"]), ExitStatusPolicy::new());
    assert_eq!(
        chain.shown(),
        [
            ("check".to_string(), "test -f /etc/a".to_string()),
            ("restart".to_string(), format!("hold {} for 60 s", path.display())),
            ("restart/systemctl".to_string(), "systemctl restart relayer.service".to_string()),
            ("after".to_string(), "systemctl is-active relayer.service".to_string()),
        ]
    );
    assert_eq!(chain.plan().operations.iter().map(|op| op.logical_id.as_str()).collect::<Vec<_>>(), ["check", "restart/systemctl", "after"]);

    let walked = chain.walk(|_| Some(0)).unwrap();
    assert_eq!(walked.iter().map(|w| w.id.as_str()).collect::<Vec<_>>(), ["check", "restart/systemctl", "restart", "after"]);
    assert_eq!(walked[2].decision.reason, format!("restart ran holding {}", path.display()));

    // a hold that cannot take its lock stops the chain
    let _other = taken_elsewhere(&path);
    let chain = LineChain::new("deploy")
        .hold(restart(&path).timeout(Duration::from_millis(100)))
        .then("after", &line(&["true"]), ExitStatusPolicy::new());
    let walked = chain.walk(|_| Some(0)).unwrap();
    assert_eq!(walked.iter().map(|w| w.id.as_str()).collect::<Vec<_>>(), ["restart"]);
}

#[test]
fn a_line_can_jump_to_a_hold() {
    let path = lock("jump");
    let chain = LineChain::new("j")
        .then("probe", &line(&["probe"]), ExitStatusPolicy::new().with_rule(ExitStatusRule::rejected_code(3, ExitDisposition::JumpTo("restart".into()), "inactive: restart it")))
        .then("skipped", &line(&["skipped"]), ExitStatusPolicy::new())
        .hold(restart(&path));
    let walked = chain.walk(|op| Some(if op.logical_id == "probe" { 3 } else { 0 })).unwrap();
    assert_eq!(walked.iter().map(|w| w.id.as_str()).collect::<Vec<_>>(), ["probe", "restart/systemctl", "restart"]);
}
