// src/builder/cp.rs

use crate::command::ShellCommand;

#[derive(Default)]
pub struct CpBuilder {
    pub from: String,
    pub to: String,
    pub recursive: bool,
    pub force: bool,
    pub verbose: bool,
}

impl CpBuilder {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            ..Default::default()
        }
    }

    pub fn recursive(mut self) -> Self { self.recursive = true; self }
    pub fn force(mut self) -> Self { self.force = true; self }
    pub fn verbose(mut self) -> Self { self.verbose = true; self }
}

impl ShellCommand for CpBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["cp".into()];
        if self.recursive { parts.push("-r".into()); }
        if self.force { parts.push("-f".into()); }
        if self.verbose { parts.push("-v".into()); }
        parts.push(self.from.clone());
        parts.push(self.to.clone());
        parts.join(" ")
    }
}



// pub struct CpBuilder {
//     pub from: String,
//     pub to: String,
//     pub recursive: bool,
//     pub force: bool,
// }
// 
// impl CpBuilder {
//     pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
//         Self {
//             from: from.into(),
//             to: to.into(),
//             recursive: false,
//             force: false,
//         }
//     }
// 
//     pub fn recursive(mut self) -> Self {
//         self.recursive = true;
//         self
//     }
// 
//     pub fn force(mut self) -> Self {
//         self.force = true;
//         self
//     }
// 
//     pub fn build(&self) -> String {
//         let mut parts = vec!["cp".to_string()];
//         if self.recursive { parts.push("-r".into()); }
//         if self.force { parts.push("-f".into()); }
//         parts.push(self.from.clone());
//         parts.push(self.to.clone());
//         parts.join(" ")
//     }
// }
// 

// use shell_commands::builder::cp::CpBuilder;
// use shell_commands::command::ShellCommand;
// 
// let cmd = CpBuilder::new("a.txt", "b.txt")
//     .recursive()
//     .force()
//     .build();
// 
// println!("Built command: {}", cmd);
// // Output: cp -r -f a.txt b.txt
