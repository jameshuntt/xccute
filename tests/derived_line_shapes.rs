//! The four shapes fleet_commands said a derived line could not hold, each
//! derived and walked through a LineChain: a repeated list, a line whose first
//! word is a value, a program with no words, and accepted exit codes. Each is
//! derived since simple_impl_derive 0.2.6, so fleet_commands' hand-made `Root`
//! and `Line` shapes can be derived lines.
#![allow(clippy::duplicated_attributes)]

use std::ffi::OsString;

use simple_impl_derive::{CompositeShell, SimpleSubCommand};
use xccute::contract::{CommandHelp, CompositeShellCommand};
use xccute::{ExitDisposition, LineChain};

#[derive(CompositeShell, Debug, Clone, Copy, Default)]
#[shell(program = "systemctl")]
#[composite(command = "stop", ty = Stop)]
#[composite(command = "is-active", method = "is_active", ty = IsActive, init = "unit")]
struct Systemctl;

/// A repeated list: `systemctl stop UNIT...`.
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "stop")]
struct Stop {
    #[arg(positional, min = 1)]
    units: Vec<OsString>,
}

/// Accepted exit codes: `systemctl is-active UNIT`, 3 named and gone on after.
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "is-active")]
#[exit_status(code = 0, word = "active")]
#[exit_status(code = 3, word = "inactive", accepted)]
struct IsActive {
    #[builder(init_required, into)]
    #[arg(positional)]
    unit: OsString,
}

#[derive(CompositeShell, Debug, Clone, Copy, Default)]
#[shell(program = "chown")]
#[composite(command = "owner", method = "owner", ty = Chown, init = "owner,path")]
struct ChownRoot;

/// A line whose first word is a value: `chown OWNER PATH`.
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(no_segment)]
struct Chown {
    #[builder(init_required, into)]
    #[arg(positional)]
    owner: OsString,
    #[builder(init_required, into)]
    #[arg(positional)]
    path: OsString,
}

/// A program with no words: `hostname`.
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(program = "hostname")]
struct Hostname;

#[test]
fn each_shape_is_a_derived_line_a_chain_walks() {
    let chain = LineChain::new("shapes")
        .then_command("stop", &Systemctl::stop().units(["a.service", "b.service"]))
        .then_command("probe", &Systemctl::is_active("a.service"))
        .then_command("own", &ChownRoot::owner("kit", "/srv/kit"))
        .then_command("name", &Hostname);
    assert_eq!(
        chain.shown().into_iter().map(|(_, line)| line).collect::<Vec<_>>(),
        ["systemctl stop a.service b.service", "systemctl is-active a.service", "chown kit /srv/kit", "hostname"]
    );
    // is-active's 3 is accepted: the chain goes on, with the line's word for it
    let walked = chain.walk(|op| Some(if op.logical_id == "probe" { 3 } else { 0 })).unwrap();
    assert_eq!(walked.len(), 4);
    assert_eq!((walked[1].decision.disposition.clone(), walked[1].decision.reason.as_str()), (ExitDisposition::Continue, "inactive"));
    assert_eq!(Hostname.build_display(), "hostname");
    assert_eq!(Stop::help_lines()[0].words, "stop UNITS...");
}
