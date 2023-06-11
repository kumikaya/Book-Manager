use actix_session::Session;
use actix_web::{web, HttpResponse};
use book_manager_service::Query;

use crate::{
    error::Error,
    handlers::{basic_context, PageParams, DEFAULT_NUMBER_PER_PAGE},
    AppState,
};

pub async fn list_books_handler(
    app_state: web::Data<AppState>,
    session: Session,
    params: web::Query<PageParams>,
) -> Result<HttpResponse, Error> {
    let template = &app_state.templates;
    let conn = &app_state.conn;
    let page = params.page.unwrap_or(1);
    let number_per_page = params.number_per_page.unwrap_or(DEFAULT_NUMBER_PER_PAGE);
    let (books, num_pages) = Query::find_books_in_page(conn, page, number_per_page).await?;
    let mut ctx = basic_context(&session)?;
    ctx.insert("title", "图书列表");
    ctx.insert("books", &books);
    ctx.insert("page", &page);
    ctx.insert("num_pages", &num_pages);
    ctx.insert("number_per_page", &number_per_page);
    let body = template.read().unwrap().render("books/list.html.tera", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
