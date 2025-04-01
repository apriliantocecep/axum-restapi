use std::net::SocketAddr;
use crate::application::security::jwt::JwtKey;

#[derive(Debug, Clone)]
pub struct Config {
    // API Configuration
    pub service_port: u16,

    // Database Configuration
    pub database_url: String,

    // JWT configuration
    pub jwt_secret: String,
    pub jwt_key: JwtKey,
    pub jwt_exp_access_token_second: i64,
    pub jwt_validation_leeway_seconds: i64,
    pub jwt_enable_revoked_tokens: bool,

    // Redis configuration
    pub redis_host: String,
    pub redis_port: u16,
}

impl Config {
    pub fn socket_addr(&self) -> SocketAddr {
        use std::str::FromStr;
        SocketAddr::from_str(&format!("{}:{}", "127.0.0.1", self.service_port)).unwrap()
    }

    pub fn redis_url(&self) -> String {
        format!("redis://{}:{}", self.redis_host, self.redis_port)
    }
}

pub fn load() -> Config {
    let env_file = ".env";
    if dotenvy::from_filename(env_file).is_ok() {
        tracing::info!("{} file loaded", env_file);
    } else {
        tracing::info!("{} file not found, using existing environment", env_file);
    }

    let jwt_secret = env_get("JWT_SECRET");

    let config = Config {
        service_port: env_parse("PORT"),
        database_url: env_get("DATABASE_URL"),
        jwt_key: JwtKey::new(jwt_secret.as_bytes()),
        jwt_secret,
        jwt_exp_access_token_second: env_parse("JWT_EXP_ACCESS_TOKEN_SECONDS"),
        jwt_validation_leeway_seconds: env_parse("JWT_VALIDATION_LEEWAY_SECONDS"),
        jwt_enable_revoked_tokens: env_parse("JWT_ENABLE_REVOKED_TOKENS"),
        redis_host: env_get("REDIS_HOST"),
        redis_port: env_parse("REDIS_PORT"),
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