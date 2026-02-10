use crate::command::ShellCommand;

#[derive(Default)]
pub struct PsqlCommandBuilder {
    pub database: Option<String>,
    pub username: Option<String>,
    pub sql: Option<String>,
    pub output: bool,
    pub quiet: bool,
}

impl PsqlCommandBuilder {
    pub fn new(sql: impl Into<String>) -> Self {
        Self {
            sql: Some(sql.into()),
            ..Default::default()
        }
    }

    pub fn database(mut self, db: impl Into<String>) -> Self {
        self.database = Some(db.into());
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.username = Some(user.into());
        self
    }

    pub fn output(mut self) -> Self {
        self.output = true;
        self
    }

    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }
}

impl ShellCommand for PsqlCommandBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["psql".to_string()];

        if let Some(db) = &self.database {
            parts.push("-d".into());
            parts.push(db.clone());
        }

        if let Some(user) = &self.username {
            parts.push("-U".into());
            parts.push(user.clone());
        }

        if self.quiet {
            parts.push("-q".into());
        }

        if let Some(sql) = &self.sql {
            parts.push("-c".into());
            parts.push(format!("\"{}\"", sql));
        }

        parts.join(" ")
    }
}

// let cmd = PsqlCommandBuilder::new("SELECT 1;")
//     .database("citusdb")
//     .user("postgres")
//     .quiet()
//     .build();
// 
// println!("{}", cmd);
// // → psql -d citusdb -U postgres -q -c "SELECT 1;"
