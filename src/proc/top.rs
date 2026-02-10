use crate::command::ShellCommand;

#[derive(Default)]
pub struct TopBuilder {
    pub batch_mode: bool,
    pub delay: Option<f32>,
    pub iterations: Option<u32>,
    pub user: Option<String>,
    pub pid: Option<u32>,
}

impl TopBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn batch_mode(mut self) -> Self {
        self.batch_mode = true;
        self
    }

    pub fn delay(mut self, seconds: f32) -> Self {
        self.delay = Some(seconds);
        self
    }

    pub fn iterations(mut self, count: u32) -> Self {
        self.iterations = Some(count);
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn pid(mut self, pid: u32) -> Self {
        self.pid = Some(pid);
        self
    }
}

impl ShellCommand for TopBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["top".to_string()];

        if self.batch_mode {
            parts.push("-b".into());
        }

        if let Some(delay) = self.delay {
            parts.push("-d".into());
            parts.push(delay.to_string());
        }

        if let Some(iters) = self.iterations {
            parts.push("-n".into());
            parts.push(iters.to_string());
        }

        if let Some(user) = &self.user {
            parts.push("-u".into());
            parts.push(user.clone());
        }

        if let Some(pid) = self.pid {
            parts.push("-p".into());
            parts.push(pid.to_string());
        }

        parts.join(" ")
    }
}

// let top_cmd = TopBuilder::new()
//     .batch_mode()
//     .delay(1.5)
//     .iterations(5)
//     .user("james")
//     .build();
// // => "top -b -d 1.5 -n 5 -u james"
