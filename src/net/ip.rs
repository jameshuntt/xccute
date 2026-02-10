use crate::command::ShellCommand;

#[derive(Debug, Clone)]
pub enum IpCommand {
    Addr,
    Link,
    Route,
    Neigh,
    Rule,
    TcpMetrics,
    Maddr,
    Monitor,
    Xfrm,
    Netns,
    Tuntap,
    Token,
    MacSec,
    Netconf,
    L2tp,
    Mroute,
    Mptcp,
    Vrfs,
    Help,
    Custom(String), // fallback
}

#[derive(Default)]
pub struct IpBuilder {
    pub command: Option<IpCommand>,
    pub args: Vec<String>,
}

impl IpBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn command(mut self, cmd: IpCommand) -> Self {
        self.command = Some(cmd);
        self
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn args(mut self, args: &[&str]) -> Self {
        self.args.extend(args.iter().map(|s| s.to_string()));
        self
    }
}

impl ShellCommand for IpBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["ip".to_string()];

        if let Some(cmd) = &self.command {
            let sub = match cmd {
                IpCommand::Addr => "addr",
                IpCommand::Link => "link",
                IpCommand::Route => "route",
                IpCommand::Neigh => "neigh",
                IpCommand::Rule => "rule",
                IpCommand::TcpMetrics => "tcp_metrics",
                IpCommand::Maddr => "maddr",
                IpCommand::Monitor => "monitor",
                IpCommand::Xfrm => "xfrm",
                IpCommand::Netns => "netns",
                IpCommand::Tuntap => "tuntap",
                IpCommand::Token => "token",
                IpCommand::MacSec => "macsec",
                IpCommand::Netconf => "netconf",
                IpCommand::L2tp => "l2tp",
                IpCommand::Mroute => "mroute",
                IpCommand::Mptcp => "mptcp",
                IpCommand::Vrfs => "vrf",
                IpCommand::Help => "help",
                IpCommand::Custom(s) => s,
            };
            parts.push(sub.to_string());
        }

        parts.extend(self.args.clone());
        parts.join(" ")
    }
}

// let ip_cmd = IpBuilder::new()
//     .command(IpCommand::Addr)
//     .args(&["show", "eth0"])
//     .build();
// // => ip addr show eth0
