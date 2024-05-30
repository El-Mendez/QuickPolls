use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Schema};
use socketioxide::SocketIo;
use crate::models::{poll, poll_answer, poll_option};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub io: SocketIo,
}

impl AppState {
    pub(crate) async fn new(io: SocketIo) -> Result<Self, DbErr> {
        let db = Database::connect("sqlite::memory:").await?;

        let builder = db.get_database_backend();
        let schema = Schema::new(builder);

        let statement1 = builder.build(&schema.create_table_from_entity(poll::Entity));
        let statement2 = builder.build(&schema.create_table_from_entity(poll_option::Entity));
        let statement3 = builder.build(&schema.create_table_from_entity(poll_answer::Entity));

        db.execute(statement1).await?;
        db.execute(statement2).await?;
        db.execute(statement3).await?;

        Ok(AppState { db, io })
    }
}
