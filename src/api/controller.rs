use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, TryIntoModel};
use crate::models::{poll, poll::Entity as Poll};

pub type Result<T> = core::result::Result<T, DbErr>;

pub struct PollController;

impl PollController {
    pub async fn create_poll(
        db: &DbConn,
        data: poll::ActiveModel,
    ) -> Result<poll::Model> {
        data
            .save(db)
            .await
            .map(TryIntoModel::try_into_model)?
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
    ) -> Result<Option<poll::Model>> {
        Poll::find_by_id(id).one(db).await
    }
}
