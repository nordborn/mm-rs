use anyhow::{Context, Result};
use redis::{Client, Commands, ConnectionLike};

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

    pub fn reconnect(&mut self) -> Result<()> {
        if !self.conn.is_open() {
            self.conn = self.client.get_connection().context("redis reconnect")?
        }
        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Result<String> {
        self.conn.get(key).context("redis get")
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        self.conn.set(key, value).context("redis set")
    }
}
