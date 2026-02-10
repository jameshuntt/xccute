# `xccute`

    framework for safe maintainable scripting

###



## return a command:
```rust
fn main() {
    let mut kill_cmd = KillBuilder::new()
        .signal("9")
        .pid(1234)
        .to_command();

    let output = kill_cmd.output();
    
    println!("{:?}", output)

    // => Ok(Output {
    //     status: ExitStatus(unix_wait_status(256)),
    //     stdout: "",
    //     stderr: "sh: line 0: kill: (1234) - No such process\n"
    // })

    // if you dont have a process running thats what the output will be
}
```
###
## return a built string:
```rust
use xxcute::KillBuilder;

fn main() {
    let kill_cmd = KillBuilder::new()
        .signal("9")
        .pid(1234)
        .build();

    println!("{}", kill_cmd)
    // => "kill -9 1234"
}
```


###
# `# soon will include`
### `# derive macros`
    derive macro to parse a clap cli to produce auto generated builders
### `# builder kit`
    builder kit to create custom cli configs
### `# better chaining`
    * more robust ways to implement chaining of commands handling
    * exit status codes builder
### `# macro rules`
    this crate was made a long time ago, it needs a makeover
    the plan is to use macro_rules to make:
        * streamlined builders
        * categorical DRY pattern macros

    i believe that by doing so i can drastically reduce not only SLoC, but also rigidity to maintaining and adding new features


    by maiking a robust builider pattern, this crate will thrive

    the publishing of the code in v0.1.1 is merely because i needed to use this crate and bulid out some other crates not in the same workspace




    ALTHOUGH NOT MUCH WILL CHANGE IN TERMS OF API
    
    DO NOT CONSIDER THIS CRATE TECHNICALLY FINALIZED IN SHAPE UNTIL V0.2.0
    THANK YOU
    
    -james