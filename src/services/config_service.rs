use crate::types::config_provider::ConfigProvider;
use crate::types::core_config::CoreConfig;

pub struct ConfigService {
    config_provider: Box<dyn ConfigProvider + Send + Sync>,
    pub core_config: CoreConfig,
}

impl ConfigService {
    pub async fn new(config_provider: Box<dyn ConfigProvider + Send + Sync>) -> ConfigService {
        let core_config = match config_provider.provide_core_config().await {
            Ok(core_config) => core_config,
            Err(_) => panic!("Provide core config failed"),
        };
        ConfigService {
            config_provider,
            core_config,
        }
    }
}