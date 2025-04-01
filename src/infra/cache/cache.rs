use redis::aio::MultiplexedConnection;
use crate::application::config::Config;
use crate::infra::cache::redis::{RedisCache, RedisOption};

pub type CacheConnection = MultiplexedConnection;

struct Cache;

impl Cache {
    pub async fn connect(config: &Config) -> MultiplexedConnection {
        let option = RedisOption {
            connection_url: config.redis_url()
        };

        RedisCache::open(Box::new(option)).await
    }
}

pub async fn load(config: &Config) -> CacheConnection {
    Cache::connect(config).await
}