//! A chain whose lines run as processes, in a test binary of its own: a child
//! started while another test holds a lock file would share that lock's open
//! file until it execs, so no lock is held in this binary.

use xccute::contract::CommandPreview;
use xccute::{ExitStatusPolicy, LineChain};

fn line(words: &[&str]) -> CommandPreview {
    CommandPreview::new(words[0].into(), words[1..].iter().map(Into::into).collect())
}

#[cfg(unix)]
#[test]
fn lines_run_as_processes() {
    let chain = LineChain::new("p").then("yes", &line(&["true"]), ExitStatusPolicy::new()).then("no", &line(&["false"]), ExitStatusPolicy::new()).then("never", &line(&["true"]), ExitStatusPolicy::new());
    let walked = chain.run().unwrap();
    assert_eq!(walked.iter().map(|w| (w.id.as_str(), w.code)).collect::<Vec<_>>(), [("yes", Some(0)), ("no", Some(1))]);
    assert_eq!(xccute::run_line(&xccute::RuntimeOperation::from_preview("x", &line(&["/no/such/program"]))), None);
}
