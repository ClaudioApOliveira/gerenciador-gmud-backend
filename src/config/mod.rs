use std::env;

use thiserror::Error;

pub mod db;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub mongodb_uri: String,
    pub database_name: String,
    pub jwt_secret: String,
    pub jwt_access_exp_minutes: u64,
    pub jwt_refresh_exp_hours: u64,
    pub auth_username: String,
    pub auth_password: String,
    pub auth_role: String,
    pub cookie_secure: bool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port_raw = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let port = port_raw
            .parse::<u16>()
            .map_err(|_| ConfigError::InvalidPort(port_raw))?;

        let mongodb_uri = required_env("MONGODB_URI")?;
        let database_name = required_env("DATABASE_NAME")?;
        let jwt_secret = required_env("JWT_SECRET")?;
        let jwt_access_exp_minutes = parse_u64_env("JWT_ACCESS_EXP_MINUTES", 15)?;
        let jwt_refresh_exp_hours = parse_u64_env("JWT_REFRESH_EXP_HOURS", 72)?;
        let auth_username = required_env("AUTH_USERNAME")?;
        let auth_password = required_env("AUTH_PASSWORD")?;
        let auth_role = env::var("AUTH_ROLE").unwrap_or_else(|_| "admin".to_string());
        let cookie_secure = parse_bool_env("COOKIE_SECURE", false)?;

        Ok(Self {
            host,
            port,
            mongodb_uri,
            database_name,
            jwt_secret,
            jwt_access_exp_minutes,
            jwt_refresh_exp_hours,
            auth_username,
            auth_password,
            auth_role,
            cookie_secure,
        })
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

fn required_env(key: &'static str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::MissingEnvVar(key))
}

fn parse_u64_env(key: &'static str, default: u64) -> Result<u64, ConfigError> {
    match env::var(key) {
        Ok(value) => value
            .parse::<u64>()
            .map_err(|_| ConfigError::InvalidEnvValue(key)),
        Err(_) => Ok(default),
    }
}

fn parse_bool_env(key: &'static str, default: bool) -> Result<bool, ConfigError> {
    match env::var(key) {
        Ok(value) => match value.to_lowercase().as_str() {
            "true" | "1" | "yes" => Ok(true),
            "false" | "0" | "no" => Ok(false),
            _ => Err(ConfigError::InvalidEnvValue(key)),
        },
        Err(_) => Ok(default),
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("variavel de ambiente obrigatoria ausente: {0}")]
    MissingEnvVar(&'static str),
    #[error("PORT invalida: {0}")]
    InvalidPort(String),
    #[error("valor invalido para variavel de ambiente: {0}")]
    InvalidEnvValue(&'static str),
}

