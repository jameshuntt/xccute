pub struct NanoBuilder {
    pub file: String,
    pub sudo: bool,
}

impl NanoBuilder {
    pub fn new(file: impl Into<String>) -> Self {
        Self { file: file.into(), sudo: false }
    }

    pub fn sudo(mut self) -> Self {
        self.sudo = true;
        self
    }

    pub fn build(&self) -> String {
        let prefix = if self.sudo { "sudo " } else { "" };
        format!("{}nano {}", prefix, self.file)
    }
}
