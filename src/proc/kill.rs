use crate::command::ShellCommand;

#[derive(Default)]
pub struct KillBuilder {
    pub signal: Option<String>,
    pub pid: Option<u32>,
    pub pids: Vec<u32>,
}

impl KillBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn signal(mut self, sig: impl Into<String>) -> Self {
        self.signal = Some(sig.into());
        self
    }

    pub fn pid(mut self, pid: u32) -> Self {
        self.pid = Some(pid);
        self
    }

    pub fn pids(mut self, list: impl Into<Vec<u32>>) -> Self {
        self.pids = list.into();
        self
    }
}

impl ShellCommand for KillBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["kill".to_string()];

        if let Some(sig) = &self.signal {
            parts.push(format!("-{}", sig));
        }

        if let Some(pid) = self.pid {
            parts.push(pid.to_string());
        }

        for pid in &self.pids {
            parts.push(pid.to_string());
        }

        parts.join(" ")
    }
}

// let kill_cmd = KillBuilder::new()
//     .signal("9")
//     .pid(1234)
//     .build();
// // => "kill -9 1234"
