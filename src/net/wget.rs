use crate::command::ShellCommand;

#[derive(Default)]
pub struct WgetBuilder {
    pub url: String,
    pub output_file: Option<String>,
    pub recursive: bool,
    pub no_clobber: bool,
    pub quiet: bool,
    pub limit_rate: Option<String>,
    pub user_agent: Option<String>,
}

impl WgetBuilder {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }

    pub fn output_file(mut self, file: impl Into<String>) -> Self {
        self.output_file = Some(file.into());
        self
    }

    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }

    pub fn no_clobber(mut self) -> Self {
        self.no_clobber = true;
        self
    }

    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    pub fn limit_rate(mut self, rate: impl Into<String>) -> Self {
        self.limit_rate = Some(rate.into());
        self
    }

    pub fn user_agent(mut self, agent: impl Into<String>) -> Self {
        self.user_agent = Some(agent.into());
        self
    }
}

impl ShellCommand for WgetBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["wget".to_string()];

        if self.recursive {
            parts.push("--recursive".into());
        }

        if self.no_clobber {
            parts.push("--no-clobber".into());
        }

        if self.quiet {
            parts.push("--quiet".into());
        }

        if let Some(limit) = &self.limit_rate {
            parts.push("--limit-rate".into());
            parts.push(limit.clone());
        }

        if let Some(agent) = &self.user_agent {
            parts.push("--user-agent".into());
            parts.push(agent.clone());
        }

        if let Some(output) = &self.output_file {
            parts.push("-O".into());
            parts.push(output.clone());
        }

        parts.push(self.url.clone());

        parts.join(" ")
    }
}

// let wget_cmd = WgetBuilder::new("https://example.com/file.tar.gz")
//     .output_file("file.tar.gz")
//     .quiet()
//     .limit_rate("500k")
//     .build();
// // => "wget --quiet --limit-rate 500k -O file.tar.gz https://example.com/file.tar.gz"
