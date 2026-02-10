use std::path::{Path, PathBuf};

use crate::{
    cargo::CargoBuilder,
    CdBuilder,
    command::ShellCommand,
    fs::{
        mkdir::MkdirBuilder,
        echo::EchoBuilder,
    },
    script_config::{
        ScriptConfig,
        CommandConfig
    },
    shell_scripts::script_builder::ScriptBuilder,
};

pub fn from_config(config: ScriptConfig) -> ScriptBuilder {
    let full_path = match &config.script.output_dir {
        Some(dir) => Path::new(dir).join(&config.script.path),
        None => PathBuf::from(&config.script.path),
    };
//    let mut builder = ScriptBuilder::new(config.script.path.clone());
    let mut builder = ScriptBuilder::new(full_path.to_string_lossy().to_string());

    for section in config.section {
        builder = builder.section(section.title);
        for cmd in section.commands {
            match cmd {
                CommandConfig::Mkdir { path, parents } => {
                    let mut cmd = MkdirBuilder::new(path);
                    if parents.unwrap_or(false) { cmd = cmd.parents(); }
                    builder = builder.add_command(cmd);
                }
                CommandConfig::Cargo { subcommand, name, bin, lib } => {
                    let mut cmd = CargoBuilder::new(subcommand).name(name);
                    if bin.unwrap_or(false) { cmd = cmd.bin(); }
                    if lib.unwrap_or(false) { cmd = cmd.lib(); }
                    builder = builder.add_command(cmd);
                }
                CommandConfig::Line { value } => {
                    builder = builder.add_line(value);
                }
                CommandConfig::Echo { message, quoted, newline } => {
                    let mut cmd = EchoBuilder::new(message);
                    if quoted.unwrap_or(false) { cmd = cmd.quoted(); }
                    if newline == Some(false) { cmd = cmd.no_newline(); }
                    builder = builder.add_command(cmd);
                }
                // match cmd {
                //     ...
                //     CommandConfig::Echo { message, quoted } => {
                //         let quoted = quoted.unwrap_or(false);
                //         let echo_cmd = if quoted {
                //             format!("ECHO_OUTPUT={:?}", message) // quoted literal
                //         } else {
                //             format!("ECHO_OUTPUT={}", message)
                //         };
                //         builder = builder.add_line(echo_cmd);
                //     }
                //     ...
                // }
                // 
                // So when you say:
                // 
                // { type = "echo", message = "..." }
                // { type = "line", value = "echo \"$ECHO_OUTPUT\" > some/file" }
                // 
                // You simulate heredoc-style writes in shell with composability.

                CommandConfig::Cd { path } => {
                    builder = builder.add_command(CdBuilder::new(path));
                }
                
            }
        }
    }

    builder
}
