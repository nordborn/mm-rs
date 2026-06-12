#[derive(Debug, Default)]
pub struct Cred {
    key: String,
    secret: String,
}

impl Cred {
    pub fn new(k: &str, s: &str) -> Self {
        Self {
            key: k.to_string(),
            secret: s.to_string(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn secret(&self) -> &str {
        &self.secret
    }
}
