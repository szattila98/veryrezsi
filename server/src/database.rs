use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

use crate::config::AppConfig;

pub async fn init(config: &AppConfig) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(config.database_url.clone());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);
    let conn = Database::connect(opt)
        .await
        .expect("Database connection failed");
    let _ = Migrator::up(&conn, None).await;
    conn
}
