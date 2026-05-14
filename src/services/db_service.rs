use std::fmt::format;
use std::ops::DerefMut;
use ormlite::{Connection, Executor};
use ormlite::sqlite::{Sqlite, SqliteConnection, SqlitePool};
use sqlx::pool::PoolConnection;
use uuid::Uuid;
use crate::services::config_service::ConfigService;
use crate::types::errors::errors::{AppError, DbConnectionError};

pub struct DbService {
    pub pool: SqlitePool,
}

impl DbService {
    pub async fn new(config_service: &ConfigService) -> DbService {
        let path = format!("{}/database/db.sqlite", config_service.core_config.appdata_path.clone());
        let pool = SqlitePool::connect(path.as_str()).await.unwrap();
        DbService {
            pool,
        }
    }

    pub async fn provide_connection(&self) -> Result<PoolConnection<Sqlite>, Box<dyn AppError>> {
        let conn = match self.pool.acquire().await {
            Ok(conn) => {conn}
            Err(_) => {return Err(DbConnectionError::new())}
        };
        Ok(conn)
    }

    pub fn create_uuid(&self) -> String {
        Uuid::new_v4().to_string()
    }
}