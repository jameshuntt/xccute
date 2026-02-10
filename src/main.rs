use xccute::{KillBuilder, ShellCommand};



fn main1() {
    let kill_cmd = KillBuilder::new()
        .signal("9")
        .pid(1234)
        .build();

    println!("{}", kill_cmd)
    // => "kill -9 1234"
}

fn main() {
    let mut kill_cmd = KillBuilder::new()
        .signal("9")
        .pid(1234)
        .to_command();

    let output = kill_cmd.output();
    println!("{:?}", output)
    // => Ok(Output { status: ExitStatus(unix_wait_status(256)), stdout: "", stderr: "sh: line 0: kill: (1234) - No such process\n" })
}