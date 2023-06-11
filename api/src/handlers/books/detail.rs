use book_manager_service::Query;
use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{error::Error, AppState, handlers::basic_context};

pub async fn book_detail_handler(
    app_state: web::Data<AppState>,
    session: Session,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let template = &app_state.templates;
    let id = id.into_inner();
    let conn = &app_state.conn;
    let book = Query::find_book_by_id(conn, id)
        .await?
        .ok_or(Error::book_not_found())?;
    let borrowed_books = Query::find_borrowed_books_detail_by_book_id(conn, id).await?;
    let date = chrono::Local::now().naive_local().date() + chrono::Duration::days(7);
    let mut ctx = basic_context(&session)?;
    ctx.insert("title", "图书详情");
    ctx.insert("book", &book);
    ctx.insert("date", &date);
    ctx.insert("borrowed_books", &borrowed_books);
    let body = template.read().unwrap().render("books/detail.html.tera", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}