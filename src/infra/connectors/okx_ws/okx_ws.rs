use crate::{
    infra::{actors::ws_market_data_provider_actor::WsMarketDataProviderActorAdapter, connectors::Okx},
    types::{traits::ExgConnector, values::Cred},
};
use anyhow::Result;
use async_trait::async_trait;
use trading_types::Symbol;

pub struct OkxWs {
    pub okx_connector: Okx,
    pub ws_market_data_provider: WsMarketDataProviderActorAdapter,
}

impl OkxWs {
    pub fn new(okx_connector: Okx, ws_market_data_provider: WsMarketDataProviderActorAdapter) -> Self {
        Self {
            okx_connector,
            ws_market_data_provider,
        }
    }
}

#[async_trait]
impl ExgConnector for OkxWs {
    async fn try_init(&mut self) -> Result<()> {
        self.on_init_subscribe_ws_private().await?;
        self.on_init_subscribe_ws_public().await?;
        Ok(())
    }
    fn set_creds(&mut self, creds: Cred) {
        self.okx_connector.set_creds(creds);
    }
    fn set_symbol(&mut self, symbol: Symbol) {
        self.okx_connector.set_symbol(symbol);
    }
}
