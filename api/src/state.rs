use log::{info, LevelFilter};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use socketioxide::SocketIo;
use crate::models::migrate::run_migrations;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub io: SocketIo,
}

impl AppState {
    pub(crate) async fn new(io: SocketIo, db_uri: &str, should_run_migrations: bool) -> Result<Self, DbErr> {
        let mut opt = ConnectOptions::new(db_uri);
        opt.sqlx_logging_level(LevelFilter::Debug);
        let db = Database::connect(opt).await?;
        info!("Database Connected!");

        if should_run_migrations {
            info!("Running Migrations");
            run_migrations(&db).await?;
            info!("Migrations done");
        } else {
            info!("Skipping migrations");
        }

        Ok(AppState { db, io })
    }
}
