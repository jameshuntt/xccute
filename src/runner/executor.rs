use crate::command::ShellCommand;
use crate::runner::error::CommandError;
use std::process::Command;

pub struct CommandChainExecutor {
    pub commands: Vec<Box<dyn ShellCommand>>,
    pub dry_run: bool,
    pub stop_on_error: bool,
}

impl CommandChainExecutor {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            dry_run: false,
            stop_on_error: true,
        }
    }

    pub fn with_dry_run(mut self, dry: bool) -> Self {
        self.dry_run = dry;
        self
    }

    pub fn with_stop_on_error(mut self, stop: bool) -> Self {
        self.stop_on_error = stop;
        self
    }

    pub fn add_command<T: ShellCommand + 'static>(mut self, command: T) -> Self {
        self.commands.push(Box::new(command));
        self
    }

    pub fn run(&self) -> Result<(), CommandError> {
        for cmd in &self.commands {
            let command_str = cmd.build();

            if self.dry_run {
                println!("[dry-run] {}", command_str);
                continue;
            }

            let status = Command::new("sh")
                .arg("-c")
                .arg(&command_str)
                .status()?;

            if !status.success() {
                if self.stop_on_error {
                    return Err(CommandError::ExecutionFailed(command_str));
                } else {
                    eprintln!("Command failed: {}", command_str);
                }
            }
        }

        Ok(())
    }
}



// use crate::ShellCommand;
// 
// use super::CommandError;
// 
// pub struct CommandChainExecutor {
//     pub commands: Vec<Box<dyn ShellCommand>>,
//     pub dry_run: bool,
//     pub parallel: bool,
// }
// 
// impl CommandChainExecutor {
//     pub fn run(&self) -> Result<(), CommandError> {
//         for cmd in &self.commands {
//             let built = cmd.build();
//             if self.dry_run {
//                 println!("[dry-run] {}", built);
//             } else {
//                 let status = std::process::Command::new("sh")
//                     .arg("-c")
//                     .arg(built)
//                     .status()?;
//                 if !status.success() {
//                     return Err(CommandError::Failed(cmd.build()));
//                 }
//             }
//         }
//         Ok(())
//     }
// }
// 