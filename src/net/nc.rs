use crate::command::ShellCommand;

#[derive(Default)]
pub struct NetcatBuilder {
    pub listen: bool,
    pub udp: bool,
    pub port: Option<u16>,
    pub host: Option<String>,
    pub input_file: Option<String>,
    pub output_file: Option<String>,
    pub zero_io: bool,
}

impl NetcatBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn listen(mut self) -> Self {
        self.listen = true;
        self
    }

    pub fn udp(mut self) -> Self {
        self.udp = true;
        self
    }

    pub fn host(mut self, h: impl Into<String>) -> Self {
        self.host = Some(h.into());
        self
    }

    pub fn port(mut self, p: u16) -> Self {
        self.port = Some(p);
        self
    }

    pub fn input(mut self, path: impl Into<String>) -> Self {
        self.input_file = Some(path.into());
        self
    }

    pub fn output(mut self, path: impl Into<String>) -> Self {
        self.output_file = Some(path.into());
        self
    }

    pub fn zero_io(mut self) -> Self {
        self.zero_io = true;
        self
    }
}

impl ShellCommand for NetcatBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["nc".to_string()];

        if self.listen {
            parts.push("-l".into());
        }

        if self.udp {
            parts.push("-u".into());
        }

        if self.zero_io {
            parts.push("-z".into());
        }

        if let Some(input) = &self.input_file {
            parts.push("-i".into());
            parts.push(input.clone());
        }

        if let Some(output) = &self.output_file {
            parts.push("-o".into());
            parts.push(output.clone());
        }

        if let Some(host) = &self.host {
            parts.push(host.clone());
        }

        if let Some(port) = self.port {
            parts.push(port.to_string());
        }

        parts.join(" ")
    }
}

// let nc_cmd = NetcatBuilder::new()
//     .host("localhost")
//     .port(8080)
//     .zero_io()
//     .build();
// // => nc -z localhost 8080
