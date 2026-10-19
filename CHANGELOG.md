# Changelog

## 0.3.1 (2026-10-19)

the first release since 0.2.1, and everything since it. 0.3.0 was tagged in git (`xccute-v0.3.0`, f3f317d) and never published; this release holds more than that tag, so it takes the next number instead of reusing the tag's. the source 0.2.1 was published from is the same as this repo's at 0.3.0; what moved since is the crates under the facade, the chain the fleet walks with a step that holds a lock, and one table for every wait.

### a chain of argv lines, with a step that holds a lock

- xccute's chain ran each step as a shell string through `sh -c` and stopped on the first code that was not 0: [src/command_chain.rs L58-L106](https://github.com/jameshuntt/xccute/blob/45ddc28eefc72dccc34b18906a276821d3972d6d/src/command_chain.rs#L58-L106). the fleet's argv lines, each read by its exit-status policy, were walked by a chain of the fleet's own, and the one step that has to hold something while it runs (fleet_ops' RestartHoldingLock: take the anchor lock within a wait, restart the unit while it is held, let it go) stayed hand-written beside it.
- new `LineChain`: argv lines (`RuntimeOperation`s, from a command's preview) each with its `ExitStatusPolicy`, walked from the first by the decision (Continue to the next step, Stop, JumpTo a named step). `then(id, preview, policy)` takes a bare preview; `then_command(id, &command)` takes a derived line with its accepted codes and its named statuses as the policy. `shown()` is each step's dry run, `plan()` every line it can run as one `RuntimeOperationPlan` for receipts, `walk(run)` answers each status from `run`, `run()` runs each line as its own process (`run_line`, argv only): [src/line_chain.rs L1-L201](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/src/line_chain.rs#L1-L201).
- new `Hold`, a step of a `LineChain`: its resource tried until it is taken or the timeout passes, a backoff apart; its own lines walked while it is held; its rollback walked, still holding it, when one of them stops on a status it does not accept; the resource let go after, either way. a resource that stays busy runs nothing and stops the chain with why (`LOCK was held for 1200 s: restart not run`). its lines are walked as `hold/line`, its rollback as `hold/rollback/line`, then the hold itself with no status of its own: [src/hold.rs L129-L252](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/src/hold.rs#L129-L252).
- a resource is `Holdable` (`name`, `try_take`: held, busy, or absent). `LockFile` is the fleet's lock: opened read-only and never created, an exclusive `flock` (`File::try_lock`); no file means nothing has taken it since its folder was made, and the lines run without it: [src/hold.rs L20-L86](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/src/hold.rs#L20-L86).

### every wait in one table

- xccute had no wait of its own. the hold's are declared once, in `xccute::timing`: `HOLD_TIMEOUT` (60 s, when a hold names none) and `HOLD_RETRY_BACKOFF` (100 ms, the fleet lock's), and `TIMING` lists them by name. a hold passes another wait to `timeout` or `retry_backoff`; nothing else in xccute holds a number of seconds. when James's time kit is a crate, xccute takes these from it: [src/timing.rs L1-L21](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/src/timing.rs#L1-L21).

### the crates under the facade

- 0.2.1 was built on xccute_contract 0.1.0, xccute_runtime 0.1.0 and xccute_decisions 0.1.0. now xccute_contract 0.1.3 (a command names its exit statuses; a command under surfaces to any depth; help lines), xccute_runtime 0.1.2 (`ExitStatusPolicy::named` and `::of`, the statuses a command names as rules), xccute_decisions 0.2.1 (its connector constructors `nodeplan` and `denv` renamed `supervisor` and `environment`; re-exported here as `xccute::decisions`): [Cargo.toml L23-L26](https://github.com/jameshuntt/xccute/blob/45ddc28eefc72dccc34b18906a276821d3972d6d/Cargo.toml#L23-L26) to [Cargo.toml L23-L26](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/Cargo.toml#L23-L26).
- simple_impl_derive 0.2.2 to 0.2.7 for the tests: [Cargo.toml L29-L29](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/Cargo.toml#L29-L29).
- rust-version 1.89, the first Rust whose `File` locks (the lock the fleet holds); 0.2.1 said 1.70, and main has said 1.85 since its CI was set up: [Cargo.toml L5-L5](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/Cargo.toml#L5-L5).
- two tests use fleet names where they used the pilot's (`fleet-worker.service`, `pgrep fleet`).

### the four derive-side gaps

- fleet_commands said a derived line could not hold a repeated list, a line whose first word is a value, a program with no words, or accepted exit codes. none still holds: simple_impl_derive 0.2.6 derives each (a `Vec` field, `no_segment`, `program`, `#[exit_status(.., accepted)]`), and a test walks each through a `LineChain`: [tests/derived_line_shapes.rs L1-L78](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/tests/derived_line_shapes.rs#L1-L78).

### proof

- a free lock held while the line runs and let go after; no lock file, nothing to wait for and never created; a lock held past the timeout runs nothing and says how long; a lock let go during the wait is taken; a failed line rolled back while the lock is still held, the lock let go after the failure too, and no rollback after a success; a hold as a step of a chain, its dry run, its plan and its stop; a line that jumps to a hold: [tests/hold_step.rs L39-L176](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/tests/hold_step.rs#L39-L176). lines run as processes, in a test binary of their own (a child shares a held lock's open file until it execs): [tests/line_chain_processes.rs L1-L19](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/tests/line_chain_processes.rs#L1-L19). the four shapes: [tests/derived_line_shapes.rs L60-L78](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/tests/derived_line_shapes.rs#L60-L78). the README example: [README.md L44-L72](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/README.md#L44-L72).
- the hold tests run 30 times in a row with no failure.
- public API against 0.2.1 (rustdoc JSON): 67 items added, none removed or changed in xccute itself; through `xccute::decisions`, xccute_decisions' rename above.
- `cargo test`: 50 passed (stable and 1.89).

### README and CI

- the chain and the hold, with an example: [README.md L44-L72](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/README.md#L44-L72).
- CI publishes from main on a version bump through crates.io trusted publishing (registered for jameshuntt/xccute, ci.yml), after the tests on stable and 1.89 and clippy: [.github/workflows/ci.yml](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/.github/workflows/ci.yml); the README's Releasing says so: [README.md L74-L77](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/README.md#L74-L77). CHANGELOG.md is in the package: [Cargo.toml L13-L13](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/Cargo.toml#L13-L13).
- version 0.3.0 to 0.3.1: [Cargo.toml L3-L3](https://github.com/jameshuntt/xccute/blob/1d206ab967625d5e67c528a76d6f99bd9d69f044/Cargo.toml#L3-L3).
