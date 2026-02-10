use crate::command::ShellCommand;

#[derive(Default)]
pub struct CurlBuilder {
    pub url: String,
    pub output: Option<String>,
    pub silent: bool,
    pub verbose: bool,
    pub follow_redirects: bool,
    pub headers: Vec<String>,
    pub request_method: Option<String>,
    pub data: Option<String>,
    pub user: Option<String>,
    pub insecure: bool,
}

impl CurlBuilder {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }

    pub fn output(mut self, path: impl Into<String>) -> Self {
        self.output = Some(path.into());
        self
    }

    pub fn silent(mut self) -> Self {
        self.silent = true;
        self
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    pub fn follow_redirects(mut self) -> Self {
        self.follow_redirects = true;
        self
    }

    pub fn header(mut self, key_value: impl Into<String>) -> Self {
        self.headers.push(key_value.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.request_method = Some(method.into());
        self
    }

    pub fn data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }

    pub fn user(mut self, creds: impl Into<String>) -> Self {
        self.user = Some(creds.into());
        self
    }

    pub fn insecure(mut self) -> Self {
        self.insecure = true;
        self
    }
}

impl ShellCommand for CurlBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["curl".to_string()];

        if self.silent {
            parts.push("-s".into());
        }

        if self.verbose {
            parts.push("-v".into());
        }

        if self.follow_redirects {
            parts.push("-L".into());
        }

        if let Some(path) = &self.output {
            parts.push("-o".into());
            parts.push(path.clone());
        }

        for header in &self.headers {
            parts.push("-H".into());
            parts.push(format!("'{}'", header));
        }

        if let Some(method) = &self.request_method {
            parts.push("-X".into());
            parts.push(method.clone());
        }

        if let Some(data) = &self.data {
            parts.push("-d".into());
            parts.push(format!("'{}'", data));
        }

        if let Some(user) = &self.user {
            parts.push("-u".into());
            parts.push(user.clone());
        }

        if self.insecure {
            parts.push("-k".into());
        }

        parts.push(self.url.clone());
        parts.join(" ")
    }
}

// let curl = CurlBuilder::new("https://example.com")
//     .silent()
//     .follow_redirects()
//     .output("out.html")
//     .header("Accept: application/json")
//     .build();
// // => curl -s -L -o out.html -H 'Accept: application/json' https://example.com
