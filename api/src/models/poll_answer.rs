use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "polls_answer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub poll_option_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::poll_option::Entity",
        from = "Column::PollOptionId",
        to = "super::poll_option::Column::Id"
    )]
    PollOption,
}


impl Related<super::poll_option::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PollOption.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
