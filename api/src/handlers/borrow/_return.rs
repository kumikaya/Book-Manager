use book_manager_service::{
    sea_orm::{self, TransactionTrait},
    Mutation, Query,
};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use migration::DbErr;

use crate::{AppState, error::Error, handlers::DeleteParams, flash_success};


#[derive(Debug)]
pub enum ReturnError {
    Err(String),
    DbError(DbErr),
}

impl ReturnError {
    pub fn new<T: ToString>(msg: T) -> Self {
        ReturnError::Err(msg.to_string())
    }
}

impl std::fmt::Display for ReturnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnError::Err(msg) => write!(f, "Borrow error: {}", msg),
            ReturnError::DbError(err) => write!(f, "Database error: {}", err),
        }
    }
}

impl std::error::Error for ReturnError {}

impl From<DbErr> for ReturnError {
    fn from(err: DbErr) -> Self {
        ReturnError::DbError(err)
    }
}

impl Into<Error> for ReturnError {
    fn into(self) -> Error {
        match self {
            ReturnError::Err(err) => Error::Other(err),
            ReturnError::DbError(err) => Error::DbErr(err),
        }
    }
}

pub async fn return_book_handler(
    app_state: web::Data<AppState>,
    session: Session,
    borrow_id: web::Path<i32>,
    params: web::Query<DeleteParams>,
) -> Result<HttpResponse, Error> {
    let conn = &app_state.conn;
    let borrow_id = borrow_id.into_inner();
    let borrowed_book = Query::find_borrowed_book_by_id(conn, borrow_id)
        .await?
        .ok_or(Error::borrow_record_not_found())?;
    let book_id = borrowed_book.book_id;
    conn.transaction::<_, (), ReturnError>(|txn| {
        Box::pin(async move {
            let book = Query::find_book_by_id(txn, book_id).await?.unwrap();
            Mutation::update_book_copies_by_id(txn, book_id, book.copies + 1).await?;
            Mutation::delete_borrowed_book(txn, borrow_id).await?;
            Ok(())
        })
    })
    .await
    .map_err(|err| match err {
        sea_orm::TransactionError::Connection(err) => Error::DbErr(err),
        sea_orm::TransactionError::Transaction(err) => err.into(),
    })?;
    let source = params.into_inner().source.unwrap_or(format!("/books/{book_id}"));
    flash_success(&session, "删除成功")?;

    Ok(HttpResponse::Found()
        .append_header(("Location", source))
        .finish())
}
