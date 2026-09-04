use serde::de::DeserializeOwned;

use std::collections::HashMap;

use crate::nested::command::NestedCommand;
use crate::nested::fragment::ShellFragment;

#[derive(Debug)]
pub enum ConfigError {
    MissingStage { key: String },
    UnknownStage { key: String },
    Toml(toml::de::Error),
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self { Self::Toml(e) }
}

pub trait DynStageFactory: Send + Sync {
    fn build(&self, value: &toml::Value) -> Result<Box<dyn ShellFragment>, ConfigError>;
}

pub struct StageFactory<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Default for StageFactory<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> StageFactory<T> {
    pub fn new() -> Self {
        Self { _phantom: std::marker::PhantomData }
    }
}

impl<T> DynStageFactory for StageFactory<T>
where
    T: StageDecl,
{
    fn build(&self, value: &toml::Value) -> Result<Box<dyn ShellFragment>, ConfigError> {
        let cfg: T::Config = value.clone().try_into()?; // uses serde via toml
        Ok(T::build(cfg))
    }
}

/// “Declaration” for a stage builder.
/// Each stage can live in its own module/crate and just implement this.
pub trait StageDecl: Send + Sync + 'static {
    type Config: DeserializeOwned;
    fn build(cfg: Self::Config) -> Box<dyn ShellFragment>;
}

pub struct NestedRegistry {
    // program -> stage_key -> factory
    programs: HashMap<String, HashMap<String, Box<dyn DynStageFactory>>>,
}

impl Default for NestedRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl NestedRegistry {
    pub fn new() -> Self {
        Self { programs: HashMap::new() }
    }

    /// Register a stage factory under:
    /// - program: "git"
    /// - key: "program", "remote", "remote.add"
    pub fn register_stage<D: StageDecl>(
        &mut self,
        program: impl Into<String>,
        key: impl Into<String>,
    ) -> &mut Self {
        let program = program.into();
        let key = key.into();

        self.programs
            .entry(program)
            .or_default()
            .insert(key, Box::new(StageFactory::<D>::new()));

        self
    }

    pub fn build_from_toml(&self, cfg: &toml::Value) -> Result<NestedCommand, ConfigError> {
        // expected shape:
        // program = "git"
        // path = ["remote","add"]
        // stages = { program = {...}, remote = {...}, remote.add = {...} }

        let program = cfg
            .get("program")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let path: Vec<String> = cfg
            .get("path")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();

        let stages = cfg.get("stages").cloned().unwrap_or_else(|| toml::Value::Table(Default::default()));

        let prog_map = self
            .programs
            .get(&program)
            .ok_or_else(|| ConfigError::UnknownStage { key: format!("program:{program}") })?;

        // helper: lookup stage table by key inside stages
        let stage_value = |k: &str| -> toml::Value {
            match &stages {
                toml::Value::Table(t) => t.get(k).cloned().unwrap_or(toml::Value::Table(Default::default())),
                _ => toml::Value::Table(Default::default()),
            }
        };

        let mut cmd = NestedCommand::new(program.clone());

        // stage "program" is optional, but common
        if let Some(factory) = prog_map.get("program") {
            let v = stage_value("program");
            cmd = cmd.push_stage(factory.build(&v)?);
        }

        // Build prefix chain: "remote", "remote.add", ...
        let mut prefix = Vec::<String>::new();
        for seg in &path {
            prefix.push(seg.clone());
            let key = prefix.join(".");

            let factory = prog_map
                .get(&key)
                .ok_or_else(|| ConfigError::UnknownStage { key: format!("{program}:{key}") })?;

            let v = stage_value(&key);
            cmd = cmd.push_stage(factory.build(&v)?);
        }

        Ok(cmd)
    }
}
