pub trait DatabaseOption {
    fn connection_url(&self) -> String;

    fn max_connections(&self) -> u32;

    fn min_connections(&self) -> u32;
}