use std::process::Command;

// src/command.rs
pub trait ShellCommand {
    fn build(&self) -> String;

    fn to_command(&self) -> Command {
        let built = self.build();
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(built);
        cmd
    }
}

pub struct RawCommand(pub String);

impl ShellCommand for RawCommand {
    fn build(&self) -> String {
        self.0.clone()
    }
}
