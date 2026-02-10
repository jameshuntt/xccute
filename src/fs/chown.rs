use crate::command::ShellCommand;

#[derive(Default)]
pub struct ChownBuilder {
    pub owner: String,
    pub group: Option<String>,
    pub target: String,
    pub recursive: bool,
    pub no_dereference: bool,
    pub from: Option<(String, Option<String>)>,
}

impl ChownBuilder {
    pub fn new(owner: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            target: target.into(),
            ..Default::default()
        }
    }

    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }

    pub fn no_dereference(mut self) -> Self {
        self.no_dereference = true;
        self
    }

    pub fn from(mut self, owner: impl Into<String>, group: Option<String>) -> Self {
        self.from = Some((owner.into(), group));
        self
    }
}

impl ShellCommand for ChownBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["chown".to_string()];
        if self.recursive { parts.push("-R".into()); }
        if self.no_dereference { parts.push("-h".into()); }

        if let Some((from_owner, from_group)) = &self.from {
            let mut from_val = from_owner.clone();
            if let Some(grp) = from_group {
                from_val.push(':');
                from_val.push_str(grp);
            }
            parts.push("--from".into());
            parts.push(from_val);
        }

        let mut ownership = self.owner.clone();
        if let Some(grp) = &self.group {
            ownership.push(':');
            ownership.push_str(grp);
        }
        parts.push(ownership);
        parts.push(self.target.clone());

        parts.join(" ")
    }
}

// let chown_cmd = ChownBuilder::new("james", "/var/log/syslog")
//     .group("wheel")
//     .recursive()
//     .build();
// // => "chown -R james:wheel /var/log/syslog"
