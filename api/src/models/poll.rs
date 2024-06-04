use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "polls")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub start_date: DateTimeWithTimeZone,
    pub end_date: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::poll_option::Entity")]
    Option,
}

impl Related<super::poll_option::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Option.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}


