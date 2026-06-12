use crate::{
    infra::{actors::ws_market_data_provider_actor::WsMarketDataProviderActorAdapter, connectors::Exmo},
    types::{traits::ExgConnector, values::Cred},
};
use anyhow::Result;
use async_trait::async_trait;
use trading_types::Symbol;

pub struct ExmoWs {
    pub exmo_connector: Exmo,
    pub ws_market_data_provider: WsMarketDataProviderActorAdapter,
}

impl ExmoWs {
    pub fn new(exmo_connector: Exmo, ws_market_data_provider: WsMarketDataProviderActorAdapter) -> Self {
        Self {
            exmo_connector,
            ws_market_data_provider,
        }
    }
}

#[async_trait]
impl ExgConnector for ExmoWs {
    async fn try_init(&mut self) -> Result<()> {
        self.on_init_subscribe_ws_private().await?;
        self.on_init_subscribe_ws_public().await?;
        Ok(())
    }
    fn set_creds(&mut self, creds: Cred) {
        self.exmo_connector.set_creds(creds);
    }
    fn set_symbol(&mut self, symbol: Symbol) {
        self.exmo_connector.set_symbol(symbol);
    }
}
