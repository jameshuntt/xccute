/// Usage:
/// shell_command_builder!(GitBuilder, "git");
/// shell_command_builder!(CargoBuilder, "cargo");

macro_rules! shell_command_builder {
    ($name:ident, $base_cmd:expr) => {
        use crate::ShellCommand;
        
        #[derive(Default)]
        pub struct $name {
            pub subcommand: Option<String>,
            pub args: Vec<String>,
        }

        impl $name {
            pub fn new(subcommand: impl Into<String>) -> Self {
                Self {
                    subcommand: Some(subcommand.into()),
                    ..Default::default()
                }
            }

            pub fn arg(mut self, arg: impl Into<String>) -> Self {
                self.args.push(arg.into());
                self
            }

            pub fn args<I, S>(mut self, args: I) -> Self
            where
                I: IntoIterator<Item = S>,
                S: Into<String>,
            {
                self.args.extend(args.into_iter().map(Into::into));
                self
            }
        }

        impl ShellCommand for $name {
            fn build(&self) -> String {
                let mut parts = vec![$base_cmd.to_string()];

                if let Some(sub) = &self.subcommand {
                    parts.push(sub.clone());
                }

                parts.extend(self.args.iter().cloned());

                parts.join(" ")
            }
        }
    };
}

// 
// shell_command_builder!(GitBuilder, "git");
// shell_command_builder!(CargoBuilder, "cargo");
// 
// let git = GitBuilder::new("clone")
//     .args(["--mirror", "https://github.com/xyz/abc.git", "abc.git"])
//     .build();
// 
// let cargo = CargoBuilder::new("new")
//     .args(["my_crate", "--lib"])
//     .build();
// 
// println!("{git}");
// println!("{cargo}");
// 



macro_rules! define_shell_builder {
    (
        $builder_name:ident,
        command = $command_str:expr,
        fields = { $( $fname:ident : $ftype:ty ),* $(,)? },
        build_parts = $build:block
    ) => {
        #[derive(Debug, Default, Clone)]
        pub struct $builder_name {
            $( pub $fname: $ftype ),*
        }

        impl $builder_name {
            pub fn new($( $fname: $ftype ),*) -> Self {
                Self { $( $fname ),* }
            }
        }

        impl ShellCommand for $builder_name {
            fn build(&self) -> String {
                let mut parts = vec![$command_str.to_string()];
                $build
                parts.join(" ")
            }
        }
    };
}
