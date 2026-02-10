use crate::command::ShellCommand;
use crate::ShBuilder;
use std::fs::{OpenOptions, read_to_string};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Default, Clone)]
pub struct ScriptBuilder {
    pub file_path: String,
    pub lines: Vec<String>,
    pub dedupe: bool,
    pub prepend: bool,
}

impl ScriptBuilder {
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
            ..Default::default()
        }
    }

    pub fn add_line(mut self, line: impl Into<String>) -> Self {
        self.lines.push(line.into());
        self
    }

    pub fn add_command<T: ShellCommand>(mut self, command: T) -> Self {
        self.lines.push(command.build());
        self
    }

    pub fn set_alias(mut self, name: impl Into<String>, command: impl Into<String>) -> Self {
        self.lines.push(format!("alias {}='{}'", name.into(), command.into()));
        self
    }

    pub fn set_export(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.lines.push(format!("export {}=\"{}\"", key.into(), value.into()));
        self
    }

    pub fn dedupe_lines(mut self) -> Self {
        self.dedupe = true;
        self
    }

    pub fn prepend(mut self) -> Self {
        self.prepend = true;
        self
    }

    pub fn write(&self) -> io::Result<()> {
        let path = Path::new(&self.file_path);
        let existing = if path.exists() {
            read_to_string(path)?
        } else {
            String::new()
        };

        let mut final_lines = if self.prepend {
            let mut new_lines = self.lines.clone();
            new_lines.push("".to_owned()); // Ensure newline before existing content
            new_lines.push(existing);
            new_lines
        } else {
            let mut old_lines: Vec<String> = existing.lines().map(|s| s.to_string()).collect();
            old_lines.push("".to_string());
            old_lines.extend(self.lines.clone());
            old_lines
        };

        if self.dedupe {
            use std::collections::HashSet;
            let mut seen = HashSet::new();
            final_lines = final_lines
                .into_iter()
                .filter(|line| {
                    if seen.contains(line) {
                        false
                    } else {
                        seen.insert(line.clone());
                        true
                    }
                })
                .collect();
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        for line in &final_lines {
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }

    pub fn add_commands<T: ShellCommand>(mut self, cmds: Vec<T>) -> Self {
        for cmd in cmds {
            self.lines.push(cmd.build());
        }
        self
    }

    pub fn add_commented_command<T: ShellCommand>(
        mut self,
        command: T,
        comment: impl Into<String>
    ) -> Self {
        let line = format!("{}  # {}", command.build(), comment.into());
        self.lines.push(line);
        self
    }

    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.lines.push(format!("# {}", comment.into()));
        self
    }
    
    pub fn section(mut self, title: impl Into<String>) -> Self {
        self.lines.push(String::new());
        self.lines.push(format!("# ===== {} =====", title.into()));
        self.lines.push(String::new());
        self
    }

    pub fn if_block<F>(mut self, condition: impl Into<String>, inner: F) -> Self
    where
        F: FnOnce(&mut ScriptBuilder),
    {
        self.lines.push(format!("if {}; then", condition.into()));
        let mut inner_builder = ScriptBuilder::default();
        inner(&mut inner_builder);
        for line in inner_builder.lines {
            self.lines.push(format!("  {}", line));
        }
        self.lines.push("fi".into());
        self
    }
    
    pub fn run_traditional(&self) -> io::Result<()> {
        for line in &self.lines {
            if line.trim().is_empty() || line.trim_start().starts_with('#') {
                continue;
            }
            let status = Command::new("sh")
                .arg("-c")
                .arg(line)
                .status()?;
            if !status.success() {
                eprintln!("Command failed: {}", line);
            }
        }
        Ok(())
    }
    
    pub fn run(&self) -> io::Result<()> {
        for line in &self.lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue; // skip empty lines and comments
            }
    
            let cmd = ShBuilder::new().command(trimmed).build();
    
            let status = std::process::Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .status()?;
    
            if !status.success() {
                eprintln!("Command failed: {}", trimmed);
            }
        }
        Ok(())
    }
    
}


