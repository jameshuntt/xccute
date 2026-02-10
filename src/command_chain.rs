use ed25519_dalek::Signer;
use std::{collections::BTreeMap, fs};

use chrono::Utc;
use ed25519_dalek::{Signature, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

use crate::{
    command::ShellCommand,
    from_config,
    CommandChainSnapshot,
    CommandError,
    RawCommand,
    ScriptConfig
};

pub struct CommandChainExecutor {
    pub steps: Vec<CommandStep>,
    pub log_file: Option<String>,
    pub dry_run: bool,
    pub on_error: Option<Box<dyn Fn(&str) + Send + Sync>>,
    pub context: Option<String>,
}

impl Default for CommandChainExecutor {
    fn default() -> Self {
        Self {
            steps: vec![],
            log_file: None,
            dry_run: false,
            on_error: None,
            context: None,
        }
    }
}

impl CommandChainExecutor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn then<T: ShellCommand + 'static>(mut self, command: T) -> Self {
        self.steps.push(CommandStep::new(Box::new(command)));
        self
    }

    pub fn when<T: ShellCommand + 'static>(
        mut self,
        condition: bool,
        command: T,
    ) -> Self {
        if condition {
            self.steps.push(CommandStep::new(Box::new(command)));
        }
        self
    }

    pub fn and_then<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        f(&mut self);
        self
    }

    pub fn run(&self) -> Result<(), CommandError> {
        for step in &self.steps {
            let cmd = step.command.build();
            println!("[EXEC] {}", cmd); // eventually support logging levels
            
            if self.dry_run {
                println!("[DRY RUN] {}", cmd);
                continue;
            }

            if let Some(ref handler) = self.on_error {
                handler(&cmd);
            }

            if let Some(ref ctx) = self.context {
                println!("\n# === {} ===", ctx);
            }            
            
            let result = std::process::Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .status();

            match result {
                Ok(status) if status.success() => continue,
                Ok(status) => return Err(CommandError::ExitFailure(status.code())),
                Err(e) => return Err(CommandError::Io(e)),
            }

            if let Some(ref log_path) = self.log_file {
                use std::fs::OpenOptions;
                use std::io::Write;
            
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(log_path)?;
            
                writeln!(file, "[EXEC] {}", cmd)?;
            }            
        }
        Ok(())
    }

    pub fn log_to_file(mut self, path: impl Into<String>) -> Self {
        self.log_file = Some(path.into());
        self
    }

    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }

    pub fn on_error<F>(mut self, f: F) -> Self
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.on_error = Some(Box::new(f));
        self
    }

    pub fn context(mut self, name: impl Into<String>) -> Self {
        self.context = Some(name.into());
        self
    }

    pub fn chain_from_toml(toml: &str) -> Result<CommandChainExecutor, Box<dyn std::error::Error>> {
        use crate::script_config::ScriptConfig;
        use crate::toml_loader::from_config;
    
        let config: ScriptConfig = toml::from_str(toml)?;
        let script_builder = from_config(config);
    
        let mut chain = CommandChainExecutor::new();
        for line in &script_builder.lines {
            if line.trim().is_empty() {
                continue;
            }
            chain = chain.then(crate::command::RawCommand(line.to_string()));
        }
    
        Ok(chain)
    }

    pub fn chain_from_toml_2(toml: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config: ScriptConfig = toml::from_str(toml)?;
        let script_builder = from_config(config);

        let mut chain = Self::new();

        for line in &script_builder.lines {
            if line.trim().is_empty() {
                continue;
            }
            chain = chain.then(RawCommand(line.to_string()));
        }

        Ok(chain)
    }

    pub fn snapshot(&self) -> CommandChainSnapshot {
        CommandChainSnapshot::new(self)
    }

    pub fn restore(snapshot: &CommandChainSnapshot) -> Self {
        let mut executor = CommandChainExecutor::default();
        executor.steps = snapshot.steps.iter().map(|s| CommandStep::new(
            Box::new(RawCommand(s.clone())))
        ).collect();
        executor.context = snapshot.context.clone();
        executor.log_file = snapshot.log_file.clone();
        executor.dry_run = snapshot.dry_run;
        executor
    }

    // pub fn sign_snapshot(&self, private_key_bytes: &[u8], path: &str) -> Result<Signature, Box<dyn std::error::Error>> {
    //     let key = SigningKey::from_bytes(private_key_bytes);
    //     let data = std::fs::read_to_string(path)?;
    //     let signature = key.sign(data.as_bytes());
    //     Ok(signature)
    // }
    pub fn sign_snapshot(&self, private_key_bytes: &[u8], path: &str) -> Result<Signature, Box<dyn std::error::Error>> {
        // Ensure the key is exactly 32 bytes
        let key = SigningKey::from_bytes(private_key_bytes.try_into().expect("Invalid private key length"));
        let data = std::fs::read_to_string(path)?;
        let signature = key.sign(data.as_bytes());
        Ok(signature)
    }
    

    pub fn verify_snapshot(path: &str, signature: &[u8], public_key_bytes: &[u8]) -> bool {
        let data = std::fs::read_to_string(path).unwrap_or_default();
        
        // let key = VerifyingKey::from_bytes(public_key_bytes).unwrap();
        let key = VerifyingKey::from_bytes(
            public_key_bytes
                .try_into()
                .expect("Public key must be 32 bytes")
        ).expect("Invalid public key");

        // let sig = Signature::from_bytes(signature).unwrap();
        // let sig = Signature::from_bytes(signature.try_into().expect("Expected 64-byte signature")).expect("Invalid signature");
        //let sig = Signature::from_bytes(signature.try_into().map_err(|_| "Expected 64-byte signature")?)?;

        let sig_bytes: [u8; 64] = hex::decode(&signature)
            .expect("Invalid signature")
            .try_into()
            .expect("Signature size mismatch");

        let sig = Signature::from_bytes(&sig_bytes);

        key.verify(data.as_bytes(), &sig).is_ok()
    }

    pub fn load_snapshot(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(path)?;
        let snapshot: CommandChainSnapshot = serde_json::from_str(&data)?;

        let mut executor = CommandChainExecutor::new()
            .dry_run();
            // .dry_run(snapshot.dry_run);

        if let Some(context) = snapshot.context {
            executor = executor.context(context);
        }

        if let Some(log_file) = snapshot.log_file {
            executor = executor.log_to_file(log_file);
        }

        for cmd in snapshot.steps {
            executor = executor.then(RawCommand(cmd));
        }

        Ok(executor)
    }
}

impl CommandChainExecutor {
    pub fn save_snapshot2(&self, path: &str, metadata: BTreeMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        // Prepare the list of command steps as strings
        let steps: Vec<String> = self.steps.iter()
            .map(|step| step.command.build())
            .collect();

        // Create the snapshot
        let mut snapshot = CommandChainSnapshot {
            timestamp: Utc::now(),
            steps,
            context: self.context.clone(),
            log_file: self.log_file.clone(),
            dry_run: self.dry_run,
            metadata,
            hash: String::new(),  // Temporary placeholder for hash
        };

        // Generate the hash
        snapshot.hash = Self::generate_snapshot_hash(&snapshot);

        // Serialize and save the snapshot
        let serialized = serde_json::to_string_pretty(&snapshot)?;
        fs::write(path, serialized)?;
        Ok(())
    }
    pub fn save_snapshot(&self, path: &str, metadata: BTreeMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        let steps: Vec<String> = self.steps.iter()
            .map(|step| step.command.build())
            .collect();

        let mut snapshot = CommandChainSnapshot {
            timestamp: Utc::now(),
            steps,
            context: self.context.clone(),
            log_file: self.log_file.clone(),
            dry_run: self.dry_run,
            metadata,
            hash: String::new(),  // Temporary placeholder for hash
        };

        // Generate the hash
        snapshot.hash = Self::generate_snapshot_hash(&snapshot);

        // Serialize and save the snapshot
        let serialized = serde_json::to_string_pretty(&snapshot)?;
        fs::write(path, serialized)?;
        Ok(())
    }
    /// Generate a SHA-256 hash for the snapshot (excluding the hash field itself)
    fn generate_snapshot_hash(snapshot: &CommandChainSnapshot) -> String {
        // Create a clone without the hash to avoid self-referential hashing
        let mut clone = snapshot.clone();
        clone.hash.clear();

        // Serialize the snapshot (excluding the hash field)
        let json = serde_json::to_string(&clone).unwrap();
        let hash = Sha256::digest(json.as_bytes());
        format!("{:x}", hash)
    }
}
pub struct CommandStep {
    pub command: Box<dyn ShellCommand>,
}

impl CommandStep {
    pub fn new(command: Box<dyn ShellCommand>) -> Self {
        Self { command }
    }
}



pub fn chain_from_toml(toml: &str) -> Result<CommandChainExecutor, Box<dyn std::error::Error>> {
    let config: ScriptConfig = toml::from_str(toml)?;
    let script = from_config(config);

    let mut chain = CommandChainExecutor::new().context("From TOML");

    for line in &script.lines {
        if line.trim().is_empty() {
            continue;
        }
        chain = chain.then(RawCommand(line.to_string()));
    }

    Ok(chain)
}





// last version
// use crate::{command::ShellCommand, CommandError, RawCommand, from_config, ScriptConfig};
// 
// pub struct CommandChainExecutor {
//     pub steps: Vec<CommandStep>,
//     pub log_file: Option<String>,
//     pub dry_run: bool,
//     pub on_error: Option<Box<dyn Fn(&str) + Send + Sync>>,
//     pub context: Option<String>,
// }
// 
// impl Default for CommandChainExecutor {
//     fn default() -> Self {
//         Self {
//             steps: vec![],
//             log_file: None,
//             dry_run: false,
//             on_error: None,
//             context: None,
//         }
//     }
// }
// 
// impl CommandChainExecutor {
//     pub fn new() -> Self {
//         Self::default()
//     }
// 
//     pub fn then<T: ShellCommand + 'static>(mut self, command: T) -> Self {
//         self.steps.push(CommandStep::new(Box::new(command)));
//         self
//     }
// 
//     pub fn when<T: ShellCommand + 'static>(mut self, condition: bool, command: T) -> Self {
//         if condition {
//             self.steps.push(CommandStep::new(Box::new(command)));
//         }
//         self
//     }
// 
//     pub fn and_then<F>(mut self, f: F) -> Self
//     where
//         F: FnOnce(&mut Self),
//     {
//         f(&mut self);
//         self
//     }
// 
//     pub fn log_to_file(mut self, path: impl Into<String>) -> Self {
//         self.log_file = Some(path.into());
//         self
//     }
// 
//     pub fn dry_run(mut self) -> Self {
//         self.dry_run = true;
//         self
//     }
// 
//     pub fn on_error<F>(mut self, f: F) -> Self
//     where
//         F: Fn(&str) + Send + Sync + 'static,
//     {
//         self.on_error = Some(Box::new(f));
//         self
//     }
// 
//     pub fn context(mut self, name: impl Into<String>) -> Self {
//         self.context = Some(name.into());
//         self
//     }
// 
//     pub fn run(&self) -> Result<(), CommandError> {
//         for step in &self.steps {
//             let cmd = step.command.build();
// 
//             if let Some(ref ctx) = self.context {
//                 println!("\n# === {} ===", ctx);
//             }
// 
//             println!("[EXEC] {}", cmd);
// 
//             if let Some(ref log_path) = self.log_file {
//                 use std::fs::OpenOptions;
//                 use std::io::Write;
// 
//                 let mut file = OpenOptions::new()
//                     .create(true)
//                     .append(true)
//                     .open(log_path)?;
// 
//                 writeln!(file, "[EXEC] {}", cmd)?;
//             }
// 
//             if self.dry_run {
//                 println!("[DRY RUN] {}", cmd);
//                 continue;
//             }
// 
//             let result = std::process::Command::new("sh")
//                 .arg("-c")
//                 .arg(&cmd)
//                 .status();
// 
//             match result {
//                 Ok(status) if status.success() => continue,
//                 Ok(status) => {
//                     if let Some(ref handler) = self.on_error {
//                         handler(&cmd);
//                     }
//                     return Err(CommandError::ExitFailure(status.code()));
//                 }
//                 Err(e) => {
//                     if let Some(ref handler) = self.on_error {
//                         handler(&cmd);
//                     }
//                     return Err(CommandError::Io(e));
//                 }
//             }
//         }
//         Ok(())
//     }
// 
//     pub fn chain_from_toml(toml: &str) -> Result<Self, Box<dyn std::error::Error>> {
//         let config: ScriptConfig = toml::from_str(toml)?;
//         let script_builder = from_config(config);
// 
//         let mut chain = Self::new();
// 
//         for line in &script_builder.lines {
//             if line.trim().is_empty() {
//                 continue;
//             }
//             chain = chain.then(RawCommand(line.to_string()));
//         }
// 
//         Ok(chain)
//     }
// }
// 
// pub struct CommandStep {
//     pub command: Box<dyn ShellCommand>,
// }
// 
// impl CommandStep {
//     pub fn new(command: Box<dyn ShellCommand>) -> Self {
//         Self { command }
//     }
// }
// 






// original
// 
// use crate::{
//     command::ShellCommand,
//     CommandError
// };
// 
// pub struct CommandChainExecutor {
//     pub steps: Vec<CommandStep>,
// }
// 
// impl Default for CommandChainExecutor {
//     fn default() -> Self {
//         Self { steps: vec![] }
//     }
// }
// 
// impl CommandChainExecutor {
//     pub fn new() -> Self {
//         Self::default()
//     }
// 
//     pub fn then<T: ShellCommand + 'static>(mut self, command: T) -> Self {
//         self.steps.push(CommandStep::new(Box::new(command)));
//         self
//     }
// 
//     pub fn when<T: ShellCommand + 'static>(
//         mut self,
//         condition: bool,
//         command: T,
//     ) -> Self {
//         if condition {
//             self.steps.push(CommandStep::new(Box::new(command)));
//         }
//         self
//     }
// 
//     pub fn and_then<F>(mut self, f: F) -> Self
//     where
//         F: FnOnce(&mut Self),
//     {
//         f(&mut self);
//         self
//     }
// 
//     pub fn run(&self) -> Result<(), CommandError> {
//         for step in &self.steps {
//             let cmd = step.command.build();
//             println!("[EXEC] {}", cmd); // eventually support logging levels
// 
//             let result = std::process::Command::new("sh")
//                 .arg("-c")
//                 .arg(cmd)
//                 .status();
// 
//             match result {
//                 Ok(status) if status.success() => continue,
//                 Ok(status) => return Err(CommandError::ExitFailure(status.code())),
//                 Err(e) => return Err(CommandError::Io(e)),
//             }
//         }
//         Ok(())
//     }
// }
// 
// pub struct CommandStep {
//     pub command: Box<dyn ShellCommand>,
// }
// 
// impl CommandStep {
//     pub fn new(command: Box<dyn ShellCommand>) -> Self {
//         Self { command }
//     }
// }
// 




// snapshot example
// fn main() {
//     let chain = CommandChainExecutor::new()
//         .then(EchoBuilder::new("Hello, world!"))
//         .log_to_file("execution.log");
// 
//     chain.save_snapshot("snapshot.json").unwrap();
// 
//     let loaded_chain = CommandChainExecutor::load_snapshot("snapshot.json").unwrap();
//     loaded_chain.run().unwrap();
// 
//     // Signing and verifying
//     let private_key = [0u8; 32]; // Replace with actual key
//     let signature = loaded_chain.sign_snapshot(&private_key, "snapshot.json").unwrap();
//     let public_key = [0u8; 32]; // Replace with actual key
// 
//     let is_valid = CommandChainExecutor::verify_snapshot("snapshot.json", &signature.to_bytes(), &public_key);
//     println!("Snapshot valid: {}", is_valid);
// }
// 