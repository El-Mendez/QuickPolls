use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, FromQueryResult, JoinType, QuerySelect, RelationTrait, TransactionTrait, TryIntoModel};
use serde::Serialize;
use crate::models::{poll, poll_option, poll_answer, poll::Entity as Poll, poll_option::Entity as PollOption};

pub type Result<T> = core::result::Result<T, DbErr>;

pub struct PollController;

#[derive(FromQueryResult, Debug, Serialize)]
pub struct PollResult {
    id: u32,
    count: u32,
}

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

    pub async fn get_results(
        db: &DbConn,
        id: u32,
    ) -> Result<Vec<PollResult>> {
        Poll::find_by_id(id)
            .select_only()
            .column_as(poll_option::Column::Id, "id")
            .column_as(poll_answer::Column::PollOptionId.count(), "count")
            .join(JoinType::InnerJoin, poll::Relation::Option.def())
            .join(JoinType::LeftJoin, poll_option::Relation::PollAnswer.def())
            .group_by(poll_option::Column::Id)
            .into_model::<PollResult>().all(db).await
    }

    pub async fn vote_poll(
        db: &DbConn,
        poll_id: u32,
        poll_option_id: u32,
    ) -> Result<Vec<PollResult>> {
        let result = PollOption::find_by_id(poll_option_id)
            .find_with_related(Poll)
            .all(db)
            .await?
            .pop()
            .map(|(option, mut poll)| (option, poll.pop().unwrap()));

        let (_, poll) = match result {
            None => return Err(DbErr::Custom("could not find poll".into())),
            Some(x) => x,
        };

        if poll.id != poll_id {
            return Err(DbErr::Custom("Poll has already ended".into()));
        }

        if poll.end_date.is_some() {
            return Err(DbErr::Custom("Poll has already ended".into()));
        }

        poll_answer::ActiveModel {
            poll_option_id: Set(poll_option_id),
            ..Default::default()
        }.save(db).await?;

        Self::get_results(db, poll_id).await
    }

    pub async fn end_poll(
        db: &DbConn,
        poll_id: u32,
    ) -> Result<Option<()>> {
        let poll = Poll::find_by_id(poll_id)
            .one(db)
            .await?;

        let poll = match poll {
            Some(x) => x,
            None => return Ok(None),
        };

        if poll.end_date.is_some() {
            return Err(DbErr::Custom("The poll has already been ended.".into()))
        };

        let mut poll: poll::ActiveModel = poll.into();

        poll.end_date = Set(Some(Utc::now().into()));
        poll.update(db).await?;
        Ok(Some(()))
    }
}
