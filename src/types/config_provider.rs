use tokio::fs;
use crate::types::core_config::CoreConfig;
use crate::types::errors::errors::{AppError, ConfigError};

#[async_trait::async_trait]
pub trait ConfigProvider {
    async fn provide_core_config(&self) -> Result<CoreConfig, Box<dyn AppError>>;
}

pub struct JsonConfigProvider {
    path: String,
}

impl JsonConfigProvider {
    pub fn new() -> Box<dyn ConfigProvider + Send + Sync> {
        Box::new(JsonConfigProvider {
            path: "/home/alx/Dev/alx-service/api/config/config.json".to_string(),
        })
    }
}

#[async_trait::async_trait]
impl ConfigProvider for JsonConfigProvider {
    async fn provide_core_config(&self) -> Result<CoreConfig, Box<dyn AppError>> {
        let config_file = match fs::read_to_string(&self.path).await {
            Ok(config_file) => config_file,
            Err(_) => return Err(ConfigError::new()),
        };
        let core_config: CoreConfig = match serde_json::from_str(config_file.as_str()) {
            Ok(core_config) => core_config,
            Err(e) => return Err(ConfigError::new()),
        };
        Ok(core_config)
    }
}