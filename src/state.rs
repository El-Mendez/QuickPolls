use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Schema};
use crate::models::{poll, poll_option};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub(crate) async fn new() -> Result<Self, DbErr> {
        let db = Database::connect("sqlite::memory:").await?;

        let builder = db.get_database_backend();
        let schema = Schema::new(builder);

        let statement1 = builder.build(&schema.create_table_from_entity(poll::Entity));
        let statement2 = builder.build(&schema.create_table_from_entity(poll_option::Entity));

        db.execute(statement1).await?;
        db.execute(statement2).await?;

        Ok(AppState { db })
    }
}
