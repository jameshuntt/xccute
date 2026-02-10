pub struct PsGrepBuilder {
    pub pattern: String,
}

impl PsGrepBuilder {
    pub fn new(pattern: impl Into<String>) -> Self {
        Self { pattern: pattern.into() }
    }

    pub fn build(&self) -> String {
        format!("ps aux | grep {}", self.pattern)
    }
}
