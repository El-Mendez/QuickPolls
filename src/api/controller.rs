use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, TransactionTrait, TryIntoModel};
use crate::models::{poll, poll_option, poll::Entity as Poll, poll_option::Entity as PollOption};

pub type Result<T> = core::result::Result<T, DbErr>;

pub struct PollController;

impl PollController {
    pub async fn create_poll(
        db: &DbConn,
        data: poll::ActiveModel,
        options: Vec<poll_option::ActiveModel>,
    ) -> Result<poll::Model> {
        let transaction = db.begin().await?;

        let poll = data
            .save(&transaction)
            .await
            .map(TryIntoModel::try_into_model)??;

        let options: Vec<_> = options.into_iter().map(|mut option| {
            option.poll_id = Set(poll.id);
            option
        }).collect();

        PollOption::insert_many(options)
            .exec(&transaction)
            .await?;

        transaction.commit().await?;

        Ok(poll)
    }

    pub async fn end_poll(
        db: &DbConn,
        id: u32,
    ) -> Result<poll::Model> {
        let mut poll: poll::ActiveModel = Poll::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("cannot find poll".to_owned()))
            .map(Into::into)?;

        poll.end_date = Set(Some(Utc::now().into()));

        poll.update(db).await
    }

    pub async fn get_poll(
        db: &DbConn,
        id: u32,
    ) -> Result<Option<(poll::Model, Vec<poll_option::Model>)>> {
        Ok(Poll::find_by_id(id)
            .find_with_related(PollOption)
            .all(db)
            .await?
            .pop())
    }
}
