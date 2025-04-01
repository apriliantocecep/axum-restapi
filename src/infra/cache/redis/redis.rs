use redis::aio::MultiplexedConnection;
use crate::infra::cache::option::CacheOption;

pub struct RedisCache;

impl RedisCache {
    pub async fn open(option: Box<dyn CacheOption>) -> MultiplexedConnection {
        match redis::Client::open(option.connection_url()) {
            Ok(redis) => match redis.get_multiplexed_async_connection().await {
                Ok(connection) => {
                    tracing::info!("connected to redis");
                    connection
                }
                Err(e) => {
                    tracing::error!("could not connect to redis: {}", e);
                    std::process::exit(1);
                }
            }
            Err(e) => {
                tracing::error!("could not open redis: {}", e);
                std::process::exit(1);
            }
        }
    }
}