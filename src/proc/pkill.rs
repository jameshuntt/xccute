pub struct PkillBuilder {
    pub signal: Option<String>,
    pub pattern: String,
}

impl PkillBuilder {
    pub fn new(pattern: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into(),
            signal: None,
        }
    }

    pub fn signal(mut self, sig: impl Into<String>) -> Self {
        self.signal = Some(sig.into());
        self
    }

    pub fn build(&self) -> String {
        match &self.signal {
            Some(sig) => format!("pkill -{} {}", sig, self.pattern),
            None => format!("pkill {}", self.pattern),
        }
    }
}
