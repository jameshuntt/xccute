# xccute

Typed shell commands for Rust.

```rust
use xccute::{fs::MkdirBuilder, proc::KillBuilder, RawCommand, ShellCommand};
use xccute::runner::executor::{CaptureMode, CommandChainExecutor};

let mkdir = MkdirBuilder::new("target/demo").parents().mode("755");
assert_eq!(mkdir.build(), "mkdir -p -m 755 target/demo");
assert_eq!(KillBuilder::new().signal("TERM").pid(1234).build(), "kill -TERM 1234");

// A chain runs commands in order; a dry run renders them and spawns nothing.
let chain = CommandChainExecutor::new()
    .with_dry_run(true)
    .with_capture(CaptureMode::Output)
    .add_command(mkdir)
    .add_command(RawCommand("echo done".to_string()));
chain.run().unwrap();
```

One facade over four crates, each usable on its own:

| crate | holds |
|---|---|
| [`xccute_commands`](https://crates.io/crates/xccute_commands) | builders for ls, cp, curl, kill, sudo, grep, git, cargo and their families; every one a `#[derive(SimpleImpl)]` struct |
| [`xccute_runtime`](https://crates.io/crates/xccute_runtime) | `ShellCommand`, `RawCommand`, `CommandSpec`, the chain executor, exit-status rules, and the connector, material and gate seams |
| [`xccute_contract`](https://crates.io/crates/xccute_contract) | argv commands under a root program: validated, previewed, approved by a policy, then executed |
| [`xccute_decisions`](https://crates.io/crates/xccute_decisions) | the questions a process asks before acting, observation tools, guided paths, runbooks |

The facade adds what sits on top: `CommandChain` with TOML-declared steps and
`CommandChainSnapshot` (hashed and ed25519-signed), `ScriptBuilder` and
`from_config` for writing shell scripts from a TOML description, `NestedCommand`
and `NestedRegistry` for staged pipelines, and `ShellProgram` for hand-written
subcommands.

Two kinds of command live here. A **shell string** (`ShellCommand::build`)
is one line joined by spaces and runs through `sh -c`; it is what the
builders render, and values in it are shell syntax. An **argv command**
(`xccute_contract`, derived with `SimpleSubCommand` and `CompositeShell`) is a
vector of `OsString`s that never meets a shell, and is validated and
previewed before it runs. Use the second for untrusted input.

## Lines walked as a chain, and a step that holds a lock

A `LineChain` walks argv lines in order, each decided by its exit-status
policy (a derived line's named statuses and accepted codes, through
`then_command`): Continue to the next step, Stop, or JumpTo a named step. A
`Hold` is a step that takes a resource, walks its own lines while it is held,
walks its rollback when one of them fails, and lets the resource go after,
either way. `LockFile` is a lock file opened read-only and never created, held
by an exclusive `flock`; no file means nothing to wait for. Every wait xccute
uses is in one table, `xccute::timing`.

```rust
use std::time::Duration;
use xccute::contract::CommandPreview;
use xccute::{ExitStatusPolicy, Hold, LineChain, LockFile};

let line = |words: &[&str]| CommandPreview::new(words[0].into(), words[1..].iter().map(Into::into).collect());
let restart = LineChain::new("restart").then("systemctl", &line(&["systemctl", "restart", "relayer.service"]), ExitStatusPolicy::new());
let undo = LineChain::new("undo").then("start", &line(&["systemctl", "start", "relayer.service"]), ExitStatusPolicy::new());
let lock = std::env::temp_dir().join("xccute-readme-no-such-dir/anchor.lock");
let hold = Hold::new("restart", LockFile::new(&lock), restart).timeout(Duration::from_secs(1200)).rollback(undo);
let chain = LineChain::new("deploy").hold(hold);

assert_eq!(chain.shown()[0].1, format!("hold {} for 1200 s", lock.display()));
// `chain.run()` runs each line as its own process; a dry walk answers each status itself
let walked = chain.walk(|_| Some(0)).unwrap();
assert_eq!(walked.iter().map(|w| w.id.as_str()).collect::<Vec<_>>(), ["restart/systemctl", "restart"]);
assert_eq!(walked[1].decision.reason, format!("restart ran no {}: nothing to wait for", lock.display()));
```

## Releasing

Bump `version` in Cargo.toml, add a dated `## <version> (YYYY-MM-DD)` entry to CHANGELOG.md, and push to
`main`. CI (.github/workflows/ci.yml) runs the tests and clippy, then publishes that version to crates.io.

## License

MIT OR Apache-2.0.
