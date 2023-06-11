use chrono::{NaiveDate, NaiveDateTime};
use sea_orm::{FromQueryResult, DeriveActiveEnum, EnumIter};
use serde::{Serialize, Deserialize};

pub mod post;
pub mod prelude;

pub mod books;
pub mod borrowed_books;
pub mod emails;
pub mod users;


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
pub enum AccessPermission {
    Admin = 0,
    User = 1,
    Guest = 2,
}

impl AccessPermission {

    pub fn is_admin(&self) -> bool {
        matches!(self, Self::Admin)
    }
}


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
pub enum EmailCategory {
    Regular = 0,
    ToAdminBroadcast = 1,
    ToUserBroadcast = 2,
}

#[derive(FromQueryResult, Serialize)]
pub struct IdResult {
    pub id: i32,
}

#[derive(FromQueryResult, Serialize)]
pub struct BorrowedBooksResult {
    pub borrow_id: i32,
    pub user_name: String,
    pub user_nickname: String,
    // pub book_id: i32,
    pub book_name: String,
    // pub book_author: String,
    pub isbn: String,
    // pub user_id: i32,
    pub borrow_date: NaiveDate,
    pub return_date: NaiveDate,
}

#[derive(Debug, FromQueryResult, Serialize)]
pub struct BorrowedBooksResultForBook {
    pub borrow_id: i32,
    pub book_id: i32,
    pub book_name: String,
    pub isbn: String,
    pub book_author: String,
    pub borrow_date: NaiveDate,
    pub return_date: NaiveDate,
}

#[derive(FromQueryResult, Serialize)]
pub struct BorrowedBooksResultForUser {
    pub borrow_id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub user_nickname: String,
    pub borrow_date: NaiveDate,
    pub return_date: NaiveDate,
}

#[derive(FromQueryResult, Serialize, Clone)]
pub struct Email {
    pub id: i32,
    pub category: EmailCategory,
    pub sender_id: i32,
    pub sender_name: String,
    pub recipient_id: i32,
    pub recipient_name: String,
    pub subject: String,
    pub content: String,
    pub date_time: NaiveDateTime,
    pub deleted_by_sender: bool,
    pub deleted_by_recipient: bool,
}

impl Into<emails::Model> for Email {
    fn into(self) -> emails::Model {
        let Email {
            id,
            category,
            sender_id,
            recipient_id,
            subject,
            content,
            date_time,
            deleted_by_sender,
            deleted_by_recipient,
            ..
        } = self;
        emails::Model {
            id,
            category,
            sender_id,
            recipient_id,
            subject,
            content,
            date_time,
            deleted_by_sender,
            deleted_by_recipient,
        }
    }
}
