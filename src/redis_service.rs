use redis::{Client, RedisError, aio::Connection, Commands};

use futures::{
    compat::Future01CompatExt,
    future::{TryFutureExt},
};

pub struct RedisService {
    client: Client,
}

impl RedisService {
    pub async fn configure(address: &str) -> RedisService {
        Client::open(address)
            .map(|client| RedisService{ client }).unwrap()
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, RedisError> {
        // workaround, make it look like we has only connection
        // when redis lib will be based on future 0.3 should be removed

        let conn = self.client.get_async_connection().compat().await?;

        let result: (Connection, Option<String>) = redis::cmd("get")
            .arg(key)
            .query_async(conn)
            .compat()
            .await?;

        Ok(result.1)
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<(), RedisError> {
        // workaround, make it look like we has only connection
        // when redis lib will be based on future 0.3 should be removed

        let conn = self.client.get_async_connection().compat().await?;

        let result: (Connection, ()) = redis::cmd("set")
            .arg(key)
            .arg(value)
            .query_async(conn)
            .compat()
            .await?;

        Ok(())
    }
}
