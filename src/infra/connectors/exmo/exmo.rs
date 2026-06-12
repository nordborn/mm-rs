use crate::types::traits::ExgConnector;

pub struct Exmo;

impl Exmo {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Exmo {
    fn default() -> Self {
        Self::new()
    }
}

impl ExgConnector for Exmo {}
