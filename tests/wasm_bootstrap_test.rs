use std::fs;
use std::path::{Path, PathBuf};

// use shell_commands::script_config::ScriptConfig;
// use shell_commands::shell_scripts::script_builder::ScriptBuilder;
use xccute::toml_loader::from_config;

/// A scratch path under the system temp dir, keyed by pid so concurrent test
/// binaries don't collide. `ScriptBuilder::write` appends into whatever the
/// file already holds — that's the builder's purpose — so the target must not
/// survive between runs. Writing into the (git-tracked) tests/output/ made the
/// committed script grow by a full copy on every run.
fn scratch_script_path() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("xccute_wasm_bootstrap_{}", std::process::id()));
    fs::create_dir_all(&dir).expect("create test output directory");
    let path = dir.join("bootstrap.sh");
    let _ = fs::remove_file(&path);
    path
}

#[test]
fn generates_wasm_server_bootstrap_script() {
    let toml_path = Path::new("script_configs/wasm_server_bootstrap.toml");
    let output_path = scratch_script_path();

    // 1. Load the TOML content
    let raw = fs::read_to_string(toml_path).expect("Failed to read TOML config");

    // 2. Parse it
    // let config: ScriptConfig = toml::from_str(&raw).expect("Failed to parse TOML into ScriptConfig");
    let config = toml::from_str(&raw).expect("Failed to parse TOML into ScriptConfig");

    // 3. Build the script from the parsed config
    // let script: ScriptBuilder = from_config(config);
    // let mut script: ScriptBuilder = from_config(config);
    let mut script = from_config(config);
    script.file_path = output_path.display().to_string();


    // 4. Write the output to a file
    script.write().expect("Failed to write shell script");

    // 5. Verify file was written and contains something
    let contents = fs::read_to_string(output_path).expect("Failed to read generated script");

    assert!(
        contents.contains("cargo new"),
        "Script output did not contain expected content"
    );
}
