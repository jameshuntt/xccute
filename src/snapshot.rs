use crate::command_chain::CommandChainExecutor;
use crate::CommandStep;
use crate::{command::ShellCommand, from_config, CommandError, RawCommand, ScriptConfig};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommandChainSnapshot {
    pub timestamp: DateTime<Utc>,
    // pub commands: Vec<String>,  // Serialized commands
    pub steps: Vec<String>,
    pub context: Option<String>,
    pub log_file: Option<String>,
    pub dry_run: bool,
    pub metadata: BTreeMap<String, String>,
    pub hash: String,
}

impl CommandChainSnapshot {
    pub fn new(executor: &CommandChainExecutor) -> Self {
        let steps = executor.steps.iter().map(|s| s.command.build()).collect();
        let context = executor.context.clone();
        let log_file = executor.log_file.clone();
        let dry_run = executor.dry_run;
        let timestamp = Utc::now();

        let snapshot = CommandChainSnapshot {
            timestamp,
            steps,
            context,
            log_file,
            dry_run,
            metadata: BTreeMap::new(),
            hash: String::new(),
        };

        snapshot.with_hash()
    }

    pub fn with_hash(mut self) -> Self {
        let json = serde_json::to_string(&self).unwrap();
        let hash = Sha256::digest(json.as_bytes());
        self.hash = format!("{:x}", hash);
        self
    }
    fn calculate_hash(&self) -> String {
        let json = serde_json::to_string(self).expect("Failed to serialize snapshot");
        let hash = Sha256::digest(json.as_bytes());
        format!("{:x}", hash)
    }
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).unwrap();
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let snapshot: Self = serde_json::from_str(&contents).unwrap();
        Ok(snapshot)
    }

    pub fn verify(&self) -> bool {
        let cloned = self.clone();
        cloned.with_hash().hash == self.hash
    }
}
