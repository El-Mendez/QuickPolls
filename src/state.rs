use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Schema};
use crate::models::poll;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub(crate) async fn new() -> Result<Self, DbErr> {
        let db = Database::connect("sqlite::memory:").await?;

        let builder = db.get_database_backend();
        let schema = Schema::new(builder);

        let statement = builder.build(&schema.create_table_from_entity(poll::Entity));
        db.execute(statement).await?;

        Ok(AppState { db })
    }
}
