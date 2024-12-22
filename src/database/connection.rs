use log::info;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::AppConfig;

pub async fn create(config: &AppConfig) -> Result<DatabaseConnection, bool> {
    let opts = ConnectOptions::new(config.database_url.clone());
    let database = Database::connect(opts).await;

    match database {
        Ok(db) => {
            info!("connected to database");

            Ok(db)
        }
        Err(e) => {
            info!("failed to connect to database: {}", e);

            Err(false)
        }
    }
}
