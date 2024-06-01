use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, Schema, TransactionTrait};
use crate::models::{poll, poll_answer, poll_option};

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let statement1 = builder.build(&schema.create_table_from_entity(poll::Entity));
    let statement2 = builder.build(&schema.create_table_from_entity(poll_option::Entity));
    let statement3 = builder.build(&schema.create_table_from_entity(poll_answer::Entity));

    let transaction = db.begin().await?;

    transaction.execute(statement1).await?;
    transaction.execute(statement2).await?;
    transaction.execute(statement3).await?;

    transaction.commit().await?;
    Ok(())
}