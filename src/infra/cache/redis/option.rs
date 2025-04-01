use crate::infra::cache::option::CacheOption;

pub struct RedisOption {
    pub connection_url: String,
}

impl CacheOption for RedisOption {
    fn connection_url(&self) -> String {
        self.connection_url.to_owned()
    }
}