use crate::command::ShellCommand;

#[derive(Default)]
pub struct SsBuilder {
    pub all: bool,
    pub listening: bool,
    pub tcp: bool,
    pub udp: bool,
    pub unix: bool,
    pub state: Option<String>,
    pub port: Option<u16>,
    pub process: bool,
}

impl SsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    pub fn listening(mut self) -> Self {
        self.listening = true;
        self
    }

    pub fn tcp(mut self) -> Self {
        self.tcp = true;
        self
    }

    pub fn udp(mut self) -> Self {
        self.udp = true;
        self
    }

    pub fn unix(mut self) -> Self {
        self.unix = true;
        self
    }

    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn process(mut self) -> Self {
        self.process = true;
        self
    }
}

impl ShellCommand for SsBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["ss".to_string()];

        if self.all { parts.push("-a".into()); }
        if self.listening { parts.push("-l".into()); }
        if self.tcp { parts.push("-t".into()); }
        if self.udp { parts.push("-u".into()); }
        if self.unix { parts.push("-x".into()); }
        if self.process { parts.push("-p".into()); }

        if let Some(state) = &self.state {
            parts.push(format!("state {}", state));
        }

        if let Some(port) = self.port {
            parts.push(format!("sport = :{}", port));
        }

        parts.join(" ")
    }
}

// let ss_cmd = SsBuilder::new()
//     .tcp()
//     .listening()
//     .process()
//     .state("ESTABLISHED")
//     .build();
// // => "ss -l -t -p state ESTABLISHED"
