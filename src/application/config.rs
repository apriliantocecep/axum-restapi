use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub service_port: u16,
    pub database_url: String,
}

impl Config {
    pub fn get_socket_addr(&self) -> SocketAddr {
        use std::str::FromStr;
        SocketAddr::from_str(&format!("{}:{}", "127.0.0.1", self.service_port)).unwrap()
    }
}

pub fn load() -> Config {
    let env_file = ".env";
    if dotenvy::from_filename(env_file).is_ok() {
        tracing::info!("{} file loaded", env_file);
    } else {
        tracing::info!("{} file not found, using existing environment", env_file);
    }

    let config = Config {
        service_port: env_parse("PORT"),
        database_url: env_get("DATABASE_URL"),
    };

    tracing::trace!("configuration: {:#?}", config);
    config
}

#[inline]
fn env_get(key: &str) -> String {
    match std::env::var(key) {
        Ok(v) => v,
        Err(e) => {
            let msg = format!("{} {}", key, e);
            tracing::error!(msg);
            std::process::exit(1);
        }
    }
}

#[inline]
#[allow(dead_code)]
fn env_get_or(key: &str, default: &str) -> String {
    if let Ok(v) = std::env::var(key) {
        return v;
    }
    default.to_owned()
}

#[inline]
fn env_parse<T: std::str::FromStr>(key: &str) -> T {
    env_get(key).parse().map_or_else(
        |_| {
            let msg = format!("failed to parse: {}", key);
            tracing::error!(msg);
            std::process::exit(1);
        },
        |v| v,
    )
}