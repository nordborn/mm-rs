use anyhow::Result;
use trading_types::Symbol;

use async_trait::async_trait;

use crate::types::{traits::ExgConnector, values::Cred};

pub struct Exmo {
    pub creds: Cred,
    pub symbol: Symbol,
}

impl Exmo {
    pub fn new(creds: Cred, symbol: Symbol) -> Self {
        Self { creds, symbol }
    }
}

#[async_trait]
impl ExgConnector for Exmo {
    async fn try_init(&mut self) -> Result<()> {
        Ok(())
    }
    fn set_creds(&mut self, creds: Cred) {
        self.creds = creds;
    }
    fn set_symbol(&mut self, symbol: Symbol) {
        self.symbol = symbol;
    }
}
