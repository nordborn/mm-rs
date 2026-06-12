use std::time::Duration;

use crate::types::{
    traits::{BotConfigProvider, ExgConnector, KVStore, StrategyCalculator},
    values::{BotConfig, BotName, SingleBotConfig, SingleBotState, SpreadExgData, StrategyCalc},
};
use anyhow::{Context, Result};

pub struct SinglePairBot {
    name: BotName,
    config: SingleBotConfig,
    state: SingleBotState,
    strategy_calculator: Box<dyn StrategyCalculator<SpreadExgData>>,
    exg_connector: Box<dyn ExgConnector>,
    config_provider: Box<dyn BotConfigProvider>,
    external_kvstore: Box<dyn KVStore>,
}

impl SinglePairBot {
    // тут ограничение 'static говорит что наша имплементация на ссылается на данные с ограниченным лайфтаймом
    pub async fn try_new(
        name: &BotName,
        strategy: Box<dyn StrategyCalculator<SpreadExgData>>,
        exg_connector: Box<dyn ExgConnector>,
        mut config_provider: Box<dyn BotConfigProvider>,
        external_kvstore: Box<dyn KVStore>,
    ) -> Result<Self> {
        let ctx = format!("SingleBot: {name}: try new");
        let config = Self::read_config(config_provider.as_mut(), name)
            .await
            .with_context(|| ctx.clone())?;
        let mut obj = Self {
            name: name.into(),
            config,
            strategy_calculator: strategy,
            exg_connector,
            state: SingleBotState::default(),
            config_provider,
            external_kvstore,
        };
        obj.try_init().await.with_context(|| ctx.clone())?;
        Ok(obj)
    }

    async fn read_config(config_provider: &mut dyn BotConfigProvider, bot_name: &BotName) -> Result<SingleBotConfig> {
        let ctx = "read_config";
        config_provider.read_configs().await.context(ctx)?;
        let config_gen = config_provider.get_config(bot_name).await.context(ctx)?.context(ctx)?;
        match config_gen {
            BotConfig::Single(config) => Ok(config),
            bad_cfg => Err(anyhow::anyhow!("bad config {bad_cfg:?}")).with_context(|| ctx),
        }
    }

    async fn try_init(&mut self) -> Result<()> {
        let ctx = "try init";
        self.exg_connector.set_symbol(self.config.symbol.clone());
        self.exg_connector.try_init().await.context(ctx)?;
        log::info!("=== DONE: {ctx} ===");
        Ok(())
    }

    pub async fn trade_loop(&mut self) -> Result<()> {
        loop {
            let _ = self.trade_iter().await.map_err(|e| log::warn!("{e}: bad iter"));
            tokio::time::sleep(Duration::from_millis(self.config.trade_iter_interval_millis)).await;
        }
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub async fn trade_iter(&mut self) -> Result<()> {
        self.state.iter += 1;
        let ctx: String = format!("{}: {}: trade_iter {}", self.name, self.config.symbol, self.state.iter);
        log::info!("{ctx}");
        self.update_config().await;
        async {
            let exg_data = self.fetch_exg_data().await?;
            let calc = self.calc(&exg_data).await?;
            self.do_actions(&calc).await?;
            self.export_trade_state(&calc).await
        }
        .await
        .with_context(|| ctx)
    }

    /// safe update: if update fails, then previous will be still in use
    pub async fn update_config(&mut self) {
        let ctx = format!("{}: update_config", &self.name);
        match Self::read_config(self.config_provider.as_mut(), &self.name).await {
            Ok(config) => self.config = config,
            Err(e) => log::warn!("{ctx}: {e:#?}: skip update"),
        }
    }

    pub async fn fetch_exg_data(&mut self) -> Result<SpreadExgData> {
        let ctx = "fetch_exg_data";
        let depth = self.exg_connector.fetch_depth().await.context(ctx)?;
        Ok(SpreadExgData { depth })
    }

    pub async fn calc(&self, exg_data: &SpreadExgData) -> Result<StrategyCalc> {
        self.strategy_calculator.calc(exg_data).context("calc")
    }

    pub async fn do_actions(&mut self, calc: &StrategyCalc) -> Result<()> {
        let symbol = &self.config.symbol;
        for order_id in calc.order_ids_to_cancel.iter() {
            let _ = self
                .exg_connector
                .cancel_order(order_id)
                .await
                .context("cancel orders")
                .map_err(|e| log::warn!("can't cancel {order_id}: {e:#?}"));
        }
        let ctx = format!("{}: {}: place orders", self.name, symbol);
        for o in calc.orders_to_place.iter() {
            match self.exg_connector.place_order(o).await.with_context(|| ctx.clone()) {
                Ok(order_id) => log::debug!("{ctx}: placed {order_id}"),
                Err(e) => log::warn!("{ctx}: can't place {o:?}: {e:#?}"),
            }
        }
        Ok(())
    }

    pub async fn export_trade_state(&mut self, calc: &StrategyCalc) -> Result<()> {
        let _ = self
            .external_kvstore
            .set(
                format!("{}:trade_state", self.name),
                serde_json::json!({
                    "state": self.state,
                    "calc": calc
                })
                .to_string(),
            )
            .await
            .map_err(|e| log::warn!("{}: export_trade_state: {e:#?}", self.name));
        Ok(())
    }
}
