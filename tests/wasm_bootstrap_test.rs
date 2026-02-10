use std::fs;
use std::path::Path;

// use shell_commands::script_config::ScriptConfig;
// use shell_commands::shell_scripts::script_builder::ScriptBuilder;
use xccute::toml_loader::from_config;

#[test]
fn generates_wasm_server_bootstrap_script() {
    let toml_path = Path::new("script_configs/wasm_server_bootstrap.toml");
    let output_path = Path::new("tests/output/bootstrap.sh");

    // 1. Load the TOML content
    let raw = fs::read_to_string(toml_path).expect("Failed to read TOML config");
    
    // 2. Parse it
    // let config: ScriptConfig = toml::from_str(&raw).expect("Failed to parse TOML into ScriptConfig");
    let config = toml::from_str(&raw).expect("Failed to parse TOML into ScriptConfig");

    // 3. Build the script from the parsed config
    // let script: ScriptBuilder = from_config(config);
    // let mut script: ScriptBuilder = from_config(config);
    let mut script = from_config(config);
    script.file_path = "tests/output/bootstrap.sh".to_string();


    // 4. Write the output to a file
    fs::create_dir_all("tests/output").expect("Failed to create output directory");
    script.write().expect("Failed to write shell script");

    // 5. Verify file was written and contains something
    let contents = fs::read_to_string(output_path).expect("Failed to read generated script");

    assert!(
        contents.contains("cargo new"),
        "Script output did not contain expected content"
    );
}
