use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "books")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub author: String,
    pub publisher: String,
    pub publish_year: NaiveDate,
    pub isbn: String,
    pub copies: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::borrowed_books::Entity")]
    BorrowedBooks,
}

impl Related<super::borrowed_books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BorrowedBooks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
