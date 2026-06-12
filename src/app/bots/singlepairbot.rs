use std::time::Duration;

use crate::{
    actors::{
        bot_config_provider_actor::{self, BOT_CONFIG_PROVIDER_DEFAULT_NAME},
        redis_store_actor, ws_datastore_actor,
    },
    infra::stores,
    types::{
        traits::{ExgConnector, SpreadStrategist},
        values::{BotConfig, BotName, Cred, SingleBotConfig, SingleBotState, SpreadExgData, StrategyCalc},
    },
};
use anyhow::{Context, Result};

pub struct SinglePairBot {
    name: BotName,
    config: SingleBotConfig,
    state: SingleBotState,
    cred: Cred,
    strategy: Box<dyn SpreadStrategist>,
    exg_connector: Box<dyn ExgConnector>,
}

impl SinglePairBot {
    // тут ограничение 'static говорит что наша имплементация на ссылается на данные с ограниченным лайфтаймом
    pub async fn try_new(
        name: &BotName,
        cred: Cred,
        strategy: impl SpreadStrategist + 'static,
        exg_connector: impl ExgConnector + 'static,
    ) -> Result<Self> {
        let ctx = format!("SingleBot: {name}: try new");
        let config = Self::read_config(name).await.with_context(|| ctx.clone())?;
        Ok(Self {
            name: name.into(),
            config,
            cred,
            strategy: Box::new(strategy),
            exg_connector: Box::new(exg_connector),
            state: SingleBotState::default(),
        })
    }

    pub async fn read_config(bot_name: &BotName) -> Result<SingleBotConfig> {
        let ctx = format!("{bot_name}: read_config");
        match bot_config_provider_actor::get_config(BOT_CONFIG_PROVIDER_DEFAULT_NAME, bot_name)
            .await
            .with_context(|| ctx.clone())?
        {
            Some(BotConfig::Single(config)) => Ok(config),
            bad_cfg => Err(anyhow::anyhow!("bad config {bad_cfg:?}")).with_context(|| ctx.clone()),
        }
    }

    // Может какие-то подготовительные действия,
    // Например, обращения к внешним API для получения дополнительного контекста
    // или расчета параметров вместо хардкода.
    // Tакже чтение и дешифровка кредов.
    pub async fn init(&mut self) -> Result<()> {
        let ctx = format!("{}: init", &self.name);
        async {
            let symbol = &self.config.symbol;
            let datastore_name = ws_datastore_actor::datastore_actor_name(symbol);
            let ws_datastore = stores::WsDatastore::new(&datastore_name);
            ws_datastore_actor::spawn_register(&datastore_name, ws_datastore).await?;
            // Говорим ему объединять диффы и снэпшоты периодически
            let dur = Duration::from_secs(10);
            ws_datastore_actor::merge_diffs_every(&datastore_name, dur).await?;
            ws_datastore_actor::export_to_redis_every(&datastore_name, dur).await?;
            self.exg_connector.subscribe_ws_public(symbol).await
        }
        .await
        .with_context(|| ctx.clone())?;
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

    pub async fn update_config(&mut self) {
        let ctx = format!("{}: update_config", &self.name);
        match Self::read_config(&self.name).await {
            Ok(config) => self.config = config,
            Err(e) => log::warn!("{ctx}: {e:#?}: skip update"),
        }
    }

    pub async fn fetch_exg_data(&self) -> Result<SpreadExgData> {
        let ctx = "fetch_exg_data";
        let depth = self.exg_connector.fetch_depth(&self.config.symbol).await.context(ctx)?;
        Ok(SpreadExgData { depth })
    }

    pub async fn calc(&self, exg_data: &SpreadExgData) -> Result<StrategyCalc> {
        self.strategy.calc(exg_data).context("calc")
    }

    pub async fn do_actions(&self, calc: &StrategyCalc) -> Result<()> {
        let symbol = &self.config.symbol;
        for order_id in calc.order_ids_to_cancel.iter() {
            let _ = self
                .exg_connector
                .cancel_order(&self.cred, order_id)
                .await
                .context("cancel orders")
                .map_err(|e| log::warn!("can't cancel {order_id}: {e:#?}"));
        }
        let ctx = format!("{}: {}: place orders", self.name, symbol);
        for o in calc.orders_to_place.iter() {
            match self
                .exg_connector
                .place_order(&self.cred, o)
                .await
                .with_context(|| ctx.clone())
            {
                Ok(order_id) => log::debug!("{ctx}: placed {order_id}"),
                Err(e) => log::warn!("{ctx}: can't place {o:?}: {e:#?}"),
            }
        }
        Ok(())
    }

    pub async fn export_trade_state(&self, calc: &StrategyCalc) -> Result<()> {
        let _ = redis_store_actor::set(
            redis_store_actor::REDIS_STORE_DEFAULT_NAME,
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
