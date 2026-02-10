use crate::{EchoBuilder, MkdirBuilder, ShellCommand};

enum CommandBuilder {
    Mkdir(MkdirBuilder),
    Echo(EchoBuilder),
    Cargo(EchoBuilder),
}

impl ShellCommand for CommandBuilder {
    fn build(&self) -> String {
        match self {
            CommandBuilder::Mkdir(cmd) => cmd.build(),
            CommandBuilder::Echo(cmd) => cmd.build(),
            CommandBuilder::Cargo(cmd) => cmd.build(),
        }
    }
}
