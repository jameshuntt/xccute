use std::process::Command;
use crate::command::ShellCommand;

#[derive(Debug, Default)]
pub struct CargoBuilder {
    pub subcommand: Option<String>,
    pub name: Option<String>,
    pub bin: bool,
    pub lib: bool,
    pub release: bool,
    pub features: Vec<String>,
    pub package: Option<String>,
    pub manifest_path: Option<String>,
    pub extra_args: Vec<String>,
}

impl CargoBuilder {
    pub fn new(subcommand: impl Into<String>) -> Self {
        Self {
            subcommand: Some(subcommand.into()),
            ..Default::default()
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn bin(mut self) -> Self {
        self.bin = true;
        self
    }

    pub fn lib(mut self) -> Self {
        self.lib = true;
        self
    }

    pub fn release(mut self) -> Self {
        self.release = true;
        self
    }

    pub fn feature(mut self, feature: impl Into<String>) -> Self {
        self.features.push(feature.into());
        self
    }

    pub fn features(mut self, features: &[&str]) -> Self {
        self.features.extend(features.iter().map(|s| s.to_string()));
        self
    }

    pub fn package(mut self, package: impl Into<String>) -> Self {
        self.package = Some(package.into());
        self
    }

    pub fn manifest_path(mut self, path: impl Into<String>) -> Self {
        self.manifest_path = Some(path.into());
        self
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.extra_args.push(arg.into());
        self
    }

    pub fn args(mut self, args: &[&str]) -> Self {
        self.extra_args.extend(args.iter().map(|s| s.to_string()));
        self
    }

    /// Convenience: add a `--target <triple>` pair.
    pub fn target(mut self, triple: impl Into<String>) -> Self {
        self.extra_args.push("--target".to_string());
        self.extra_args.push(triple.into());
        self
    }

    /// Turn this into a `Command` you can run.
    /// NOTE: assumes no spaces *inside* args (which is true for how you’re using it).
    pub fn to_command(&self) -> Command {
        let cmdline = self.build();
        let mut parts = cmdline.split_whitespace();
        let program = parts.next().unwrap_or("cargo");
        let mut cmd = Command::new(program);
        cmd.args(parts);
        cmd
    }
}

impl ShellCommand for CargoBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["cargo".to_string()];

        if let Some(cmd) = &self.subcommand {
            parts.push(cmd.clone());
        }

        if let Some(name) = &self.name {
            parts.push(name.clone());
        }

        if self.bin {
            parts.push("--bin".to_string());
        }

        if self.lib {
            parts.push("--lib".to_string());
        }

        if self.release {
            parts.push("--release".to_string());
        }

        if let Some(pkg) = &self.package {
            parts.push("--package".to_string());
            parts.push(pkg.clone());
        }

        if let Some(path) = &self.manifest_path {
            parts.push("--manifest-path".to_string());
            parts.push(path.clone());
        }

        if !self.features.is_empty() {
            parts.push("--features".to_string());
            parts.push(self.features.join(","));
        }

        parts.extend(self.extra_args.clone());
        parts.join(" ")
    }
}


// use crate::command::ShellCommand;
// 
// #[derive(Debug, Default)]
// pub struct CargoBuilder {
//     pub subcommand: Option<String>,
//     pub name: Option<String>,
//     pub bin: bool,
//     pub lib: bool,
//     pub release: bool,
//     pub features: Vec<String>,
//     pub package: Option<String>,
//     pub manifest_path: Option<String>,
//     pub extra_args: Vec<String>,
// }
// 
// impl CargoBuilder {
//     pub fn new(subcommand: impl Into<String>) -> Self {
//         Self {
//             subcommand: Some(subcommand.into()),
//             ..Default::default()
//         }
//     }
// 
//     pub fn name(mut self, name: impl Into<String>) -> Self {
//         self.name = Some(name.into());
//         self
//     }
// 
//     pub fn bin(mut self) -> Self {
//         self.bin = true;
//         self
//     }
// 
//     pub fn lib(mut self) -> Self {
//         self.lib = true;
//         self
//     }
// 
//     pub fn release(mut self) -> Self {
//         self.release = true;
//         self
//     }
// 
//     pub fn feature(mut self, feature: impl Into<String>) -> Self {
//         self.features.push(feature.into());
//         self
//     }
// 
//     pub fn features(mut self, features: &[&str]) -> Self {
//         self.features.extend(features.iter().map(|s| s.to_string()));
//         self
//     }
// 
//     pub fn package(mut self, package: impl Into<String>) -> Self {
//         self.package = Some(package.into());
//         self
//     }
// 
//     pub fn manifest_path(mut self, path: impl Into<String>) -> Self {
//         self.manifest_path = Some(path.into());
//         self
//     }
// 
//     pub fn arg(mut self, arg: impl Into<String>) -> Self {
//         self.extra_args.push(arg.into());
//         self
//     }
// 
//     pub fn args(mut self, args: &[&str]) -> Self {
//         self.extra_args.extend(args.iter().map(|s| s.to_string()));
//         self
//     }
// }
// 
// impl ShellCommand for CargoBuilder {
//     fn build(&self) -> String {
//         let mut parts = vec!["cargo".to_string()];
// 
//         if let Some(cmd) = &self.subcommand {
//             parts.push(cmd.clone());
//         }
// 
//         if let Some(name) = &self.name {
//             parts.push(name.clone());
//         }
// 
//         if self.bin {
//             parts.push("--bin".to_string());
//         }
// 
//         if self.lib {
//             parts.push("--lib".to_string());
//         }
// 
//         if self.release {
//             parts.push("--release".to_string());
//         }
// 
//         if let Some(pkg) = &self.package {
//             parts.push("--package".to_string());
//             parts.push(pkg.clone());
//         }
// 
//         if let Some(path) = &self.manifest_path {
//             parts.push("--manifest-path".to_string());
//             parts.push(path.clone());
//         }
// 
//         if !self.features.is_empty() {
//             parts.push("--features".to_string());
//             parts.push(self.features.join(","));
//         }
// 
//         parts.extend(self.extra_args.clone());
// 
//         parts.join(" ")
//     }
// }


// let cmd = CargoBuilder::new("build")
//     .release()
//     .features(&["wasm", "json"])
//     .build();
// 
// println!("{}", cmd);
// // → cargo build --release --features wasm,json

// let cmd = CargoBuilder::new("new")
//     .name("my_crate")
//     .lib()
//     .build();
// 
// println!("{}", cmd);
// // → cargo new my_crate --lib

// let cmd = CargoBuilder::new("run")
//     .package("core-lib")
//     .manifest_path("path/to/Cargo.toml")
//     .build();
// 
// println!("{}", cmd);
// // → cargo run --package core-lib --manifest-path path/to/Cargo.toml
