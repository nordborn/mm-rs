use anyhow::{Context, Result};
use async_trait::async_trait;
use redis::{Client, Commands, ConnectionLike};

use crate::types::traits::KVStore;

pub struct RedisStore {
    client: redis::Client,
    conn: redis::Connection,
}

impl RedisStore {
    pub fn try_new(store_url: &str) -> Result<Self> {
        let ctx = format!("redis store {store_url}");
        let client = Client::open(store_url).with_context(|| ctx.clone())?;
        let conn = client.get_connection().with_context(|| ctx)?;
        Ok(Self { client, conn })
    }
}

#[async_trait]
impl KVStore for RedisStore {
    async fn get(&mut self, key: String) -> Result<String> {
        self.conn.get(key).context("redis get")
    }

    async fn set(&mut self, key: String, value: String) -> Result<()> {
        self.conn.set(key, value).context("redis set")
    }

    fn reconnect(&mut self) -> Result<()> {
        if !self.conn.is_open() {
            self.conn = self.client.get_connection().context("redis reconnect")?
        }
        Ok(())
    }
}
