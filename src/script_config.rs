use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScriptConfig {
    pub script: ScriptMeta,
    pub section: Vec<ScriptSection>,
}

#[derive(Debug, Deserialize)]
pub struct ScriptMeta {
    pub path: String,
    pub output_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScriptSection {
    pub title: String,
    pub commands: Vec<CommandConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum CommandConfig {
    #[serde(rename = "mkdir")]
    Mkdir { path: String, parents: Option<bool> },

    #[serde(rename = "cargo")]
    Cargo {
        subcommand: String,
        name: String,
        bin: Option<bool>,
        lib: Option<bool>,
    },

    #[serde(rename = "line")]
    Line { value: String },

    #[serde(rename = "echo")]
    Echo {
        message: String,
        quoted: Option<bool>,
        newline: Option<bool>
    },

    #[serde(rename = "cd")]
    Cd { path: String }    
    
}
