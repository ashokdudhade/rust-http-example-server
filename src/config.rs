use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub environment: String,
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub json_format: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub enabled: bool,
    pub origins: Vec<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let mut settings = config::Config::builder()
            .add_source(config::File::with_name("config/default").required(false))
            .add_source(config::File::with_name("config/local").required(false))
            .add_source(config::Environment::with_prefix("APP").separator("__"));

        // Add environment-specific config
        if let Ok(env) = std::env::var("APP_ENVIRONMENT") {
            settings = settings.add_source(
                config::File::with_name(&format!("config/{}", env)).required(false),
            );
        }

        let config = settings.build()?.try_deserialize().unwrap_or_else(|_| {
            // If deserialization fails, use default config
            AppConfig::default()
        });

        Ok(config)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        let host = std::env::var("APP_SERVER__HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("APP_SERVER__PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);

        Self {
            environment: std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            server: ServerConfig {
                address: format!("{}:{}", host, port),
                host,
                port,
            },
            logging: LoggingConfig {
                level: std::env::var("APP_LOGGING__LEVEL").unwrap_or_else(|_| "info".to_string()),
                json_format: std::env::var("APP_LOGGING__JSON_FORMAT")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
            },
            cors: CorsConfig {
                enabled: std::env::var("APP_CORS__ENABLED")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                origins: std::env::var("APP_CORS__ORIGINS")
                    .unwrap_or_else(|_| "*".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            },
        }
    }
}