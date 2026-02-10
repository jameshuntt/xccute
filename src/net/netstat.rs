use crate::command::ShellCommand;

#[derive(Default)]
pub struct NetstatBuilder {
    pub tcp: bool,
    pub udp: bool,
    pub listening: bool,
    pub programs: bool,
    pub numeric: bool,
    pub routes: bool,
    pub all: bool,
    pub interfaces: bool,
}

impl NetstatBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tcp(mut self) -> Self {
        self.tcp = true;
        self
    }

    pub fn udp(mut self) -> Self {
        self.udp = true;
        self
    }

    pub fn listening(mut self) -> Self {
        self.listening = true;
        self
    }

    pub fn programs(mut self) -> Self {
        self.programs = true;
        self
    }

    pub fn numeric(mut self) -> Self {
        self.numeric = true;
        self
    }

    pub fn routes(mut self) -> Self {
        self.routes = true;
        self
    }

    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    pub fn interfaces(mut self) -> Self {
        self.interfaces = true;
        self
    }
}

impl ShellCommand for NetstatBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["netstat".to_string()];

        if self.tcp {
            parts.push("-t".to_string());
        }

        if self.udp {
            parts.push("-u".to_string());
        }

        if self.listening {
            parts.push("-l".to_string());
        }

        if self.programs {
            parts.push("-p".to_string());
        }

        if self.numeric {
            parts.push("-n".to_string());
        }

        if self.routes {
            parts.push("-r".to_string());
        }

        if self.all {
            parts.push("-a".to_string());
        }

        if self.interfaces {
            parts.push("-i".to_string());
        }

        parts.join(" ")
    }
}

// let netstat_cmd = NetstatBuilder::new()
//     .tcp()
//     .udp()
//     .listening()
//     .programs()
//     .numeric()
//     .build();
// // => netstat -t -u -l -p -n
