use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_origin: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub create_if_missing: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_content = fs::read_to_string("config.toml")
            .map_err(|e| anyhow::anyhow!("无法读取配置文件 config.toml: {}", e))?;
        
        let config: Config = toml::from_str(&config_content)
            .map_err(|e| anyhow::anyhow!("配置文件格式错误: {}", e))?;
        
        Ok(config)
    }
    
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
    
    pub fn server_url(&self) -> String {
        format!("http://localhost:{}", self.server.port)
    }
}