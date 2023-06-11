use book_manager_service::{
    sea_orm::{self, DatabaseConnection, TransactionTrait},
    Mutation, Query,
};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use chrono::NaiveDate;
use entity::borrowed_books;
use migration::DbErr;
use serde::Deserialize;

use crate::{error::Error, AppState, flash_error, flash_success};

#[derive(Deserialize)]
pub struct BorrowBookForm {
    pub user_name: String,
    pub return_date: chrono::NaiveDate,
}

#[derive(Debug)]
pub enum BorrowError {
    Err(String),
    DatabaseError(DbErr),
}

impl BorrowError {
    pub fn new<T: ToString>(msg: T) -> Self {
        BorrowError::Err(msg.to_string())
    }
}

impl std::fmt::Display for BorrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BorrowError::Err(msg) => write!(f, "Borrow error: {}", msg),
            BorrowError::DatabaseError(err) => write!(f, "Database error: {}", err),
        }
    }
}

impl std::error::Error for BorrowError {}

impl From<DbErr> for BorrowError {
    fn from(err: DbErr) -> Self {
        BorrowError::DatabaseError(err)
    }
}

pub async fn borrow_book_post_handler(
    app_state: web::Data<AppState>,
    session: Session,
    book_id: web::Path<i32>,
    post_form: web::Form<BorrowBookForm>,
) -> Result<HttpResponse, Error> {
    let BorrowBookForm {
        user_name,
        return_date,
    } = post_form.into_inner();

    let book_id = book_id.into_inner();

    // 使用一个单独的函数将HTTP响应构建成一个闭包，这样可以重复使用
    let http_response = || {
        HttpResponse::Found()
            .append_header(("Location", format!("/books/{book_id}", book_id = book_id)))
            .finish()
    };

    let conn = &app_state.conn;

    // 通过用户名寻找用户，如果未找到用户则返回错误信息
    if let Some(user) = Query::find_user_by_name(conn, &user_name).await? {
        // 尝试借阅图书，如果出错则返回错误信息
        match borrow_book(conn, user.id, book_id, return_date).await {
            Ok(_) => {
                flash_success(&session, "借阅成功")?;
                Ok(http_response())
            }
            Err(err) => {
                flash_error(&session, format!("借阅图书失败，错误信息：{}", err))?;
                Ok(http_response())
            }
        }
    } else {
        flash_error(&session, "未找到用户")?;
        Ok(http_response())
    }
}

async fn borrow_book(
    conn: &DatabaseConnection,
    user_id: i32,
    book_id: i32,
    return_date: chrono::NaiveDate,
) -> Result<(), BorrowError> {
    conn.transaction::<_, (), BorrowError>(|txn| {
        Box::pin(async move {
            let borrowed_books = Query::find_borrowed_books_by_user_id(txn, user_id).await?;

            let current_date = chrono::Local::now().naive_local().date();
            can_borrow_book(&borrowed_books, current_date, return_date)
                .map_err(BorrowError::Err)?;

            let book = Query::find_book_by_id(txn, book_id).await?;
            match book {
                Some(book) if book.copies > 0 => {
                    Mutation::update_book_copies_by_id(txn, book_id, book.copies - 1).await?;
                }
                Some(_) => return Err(BorrowError::Err("没有库存了".to_owned())),
                None => return Err(BorrowError::Err("没有这本书".to_owned())),
            }

            Mutation::create_borrowed_book(txn, user_id, book_id, current_date, return_date)
                .await?;
            Ok(())
        })
    })
    .await
    .map_err(|err| match err {
        sea_orm::TransactionError::Connection(err) => BorrowError::DatabaseError(err),
        sea_orm::TransactionError::Transaction(err) => err,
    })
}

fn can_borrow_book(
    borrowed_books: &[borrowed_books::Model],
    current_date: NaiveDate,
    return_date: NaiveDate,
) -> Result<(), String> {
    if return_date < current_date {
        return Err(String::from("归还日期不能早于当前日期"));
    }
    if borrowed_books.len() >= 8 {
        return Err(String::from("最多只能借8本书"));
    }
    if (return_date - current_date).num_days() > 30 {
        return Err(String::from("无法一次性借书超过30天"));
    }
    for book in borrowed_books {
        if book.return_date < current_date {
            return Err(String::from("有逾期未还的书籍"));
        }
    }
    Ok(())
}
