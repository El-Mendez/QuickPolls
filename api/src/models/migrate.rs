use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, Schema, TransactionTrait};
use crate::models::{poll, poll_answer, poll_option};

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let t = schema.create_table_from_entity(poll::Entity).if_not_exists().to_owned();
    let statement1 = builder.build(&t);

    let t = schema.create_table_from_entity(poll_option::Entity).if_not_exists().to_owned();
    let statement2 = builder.build(&t);

    let t = schema.create_table_from_entity(poll_answer::Entity).if_not_exists().to_owned();
    let statement3 = builder.build(&t);

    let transaction = db.begin().await?;
    transaction.execute(statement1).await?;
    transaction.execute(statement2).await?;
    transaction.execute(statement3).await?;
    transaction.commit().await?;
    Ok(())
}