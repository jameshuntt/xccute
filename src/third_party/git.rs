use crate::command::ShellCommand;

#[derive(Default)]
pub struct GitBuilder {
    pub subcommand: Option<String>,
    pub args: Vec<String>,
}

impl GitBuilder {
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

impl GitBuilder {
    pub fn clone_mirror(url: &str, path: &str) -> Self {
        GitBuilder::new("clone").args(["--mirror", url, path])
    }

    pub fn clone_working(mirror_dir: &str, working_copy: &str) -> Self {
        GitBuilder::new("clone").args([mirror_dir, working_copy])
    }

    pub fn worktree_add(tag: &str, dest: &str) -> Self {
        GitBuilder::new("worktree").args(["add", "--detach", dest, tag])
    }
}


impl ShellCommand for GitBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["git".to_string()];

        if let Some(sub) = &self.subcommand {
            parts.push(sub.clone());
        }

        parts.extend(self.args.iter().cloned());

        parts.join(" ")
    }
}

// Example usage:
// let git_cmd = GitBuilder::new("clone")
//     .args(["--mirror", "https://github.com/user_or_org/repo.git", "repo.git"])
//     .build();
// println!("{}", git_cmd);
// // → git clone --mirror https://github.com/user_or_org/repo.git repo.git
