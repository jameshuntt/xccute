use crate::command::ShellCommand;

#[derive(Default)]
pub struct UptimeBuilder;

impl UptimeBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl ShellCommand for UptimeBuilder {
    fn build(&self) -> String {
        "uptime".into()
    }
}

// let cmd1 = UptimeBuilder::new().build();
// // => "uptime"
