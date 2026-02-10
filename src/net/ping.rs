use crate::command::ShellCommand;

#[derive(Default)]
pub struct PingBuilder {
    pub host: String,
    pub count: Option<u32>,
    pub interval: Option<f32>,
    pub timeout: Option<u32>,
    pub packet_size: Option<u32>,
    pub ipv6: bool,
    pub quiet: bool,
}

impl PingBuilder {
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            ..Default::default()
        }
    }

    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    pub fn interval(mut self, interval: f32) -> Self {
        self.interval = Some(interval);
        self
    }

    pub fn timeout(mut self, timeout: u32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn packet_size(mut self, size: u32) -> Self {
        self.packet_size = Some(size);
        self
    }

    pub fn ipv6(mut self) -> Self {
        self.ipv6 = true;
        self
    }

    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }
}

impl ShellCommand for PingBuilder {
    fn build(&self) -> String {
        let mut parts = vec![if self.ipv6 { "ping6" } else { "ping" }.to_string()];

        if let Some(c) = self.count {
            parts.push("-c".into());
            parts.push(c.to_string());
        }

        if let Some(i) = self.interval {
            parts.push("-i".into());
            parts.push(i.to_string());
        }

        if let Some(t) = self.timeout {
            parts.push("-W".into());
            parts.push(t.to_string());
        }

        if let Some(s) = self.packet_size {
            parts.push("-s".into());
            parts.push(s.to_string());
        }

        if self.quiet {
            parts.push("-q".into());
        }

        parts.push(self.host.clone());

        parts.join(" ")
    }
}

// let ping_cmd = PingBuilder::new("8.8.8.8")
//     .count(4)
//     .timeout(2)
//     .quiet()
//     .build();
// // => ping -c 4 -W 2 -q 8.8.8.8
