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

## License

MIT OR Apache-2.0.
