pub trait CacheOption {
    fn connection_url(&self) -> String;
}