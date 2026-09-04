use xccute::{CommandSpec, ShellCommand};
use xccute::runner::executor::CaptureMode;
use xccute::runner::error::CommandError;
use xccute::status::OutputExt;

#[test]
fn xccute_facade_keeps_runtime_module_imports_available() {
    let spec = CommandSpec::new("git").path(["status"]).arg("--short");

    assert_eq!(spec.build(), "git status --short");
    assert!(matches!(CaptureMode::StatusOnly, CaptureMode::StatusOnly));

    let err = CommandError::ExecutionFailed("demo".to_string());
    assert!(err.to_string().contains("demo"));
}

#[test]
fn output_ext_still_imports_through_xccute_status_module() {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("printf hello")
        .output()
        .expect("sh should run in facade compatibility test");

    assert_eq!(output.code(), Some(0));
    assert_eq!(output.stdout_text(), "hello");
}
