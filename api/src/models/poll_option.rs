use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "polls_option")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub poll_id: i32,
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::poll::Entity",
        from = "Column::PollId",
        to = "super::poll::Column::Id"
    )]
    Poll,
    #[sea_orm(has_many = "super::poll_answer::Entity")]
    PollAnswer,
}


impl Related<super::poll::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Poll.def()
    }
}

impl Related<super::poll_answer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PollAnswer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
