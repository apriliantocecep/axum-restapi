use crate::infra::database::option::DatabaseOption;

pub struct PostgresOptions {
    pub connection_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl DatabaseOption for PostgresOptions {
    fn connection_url(&self) -> String {
        self.connection_url.to_owned()
    }

    fn max_connections(&self) -> u32 {
        self.max_connections
    }

    fn min_connections(&self) -> u32 {
        self.min_connections
    }
}