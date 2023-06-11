use book_manager_service::Query;
use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{
    error::Error,
    handlers::{basic_context, is_admin, PageParams, DEFAULT_NUMBER_PER_PAGE},
    AppState,
};

pub async fn list_borrowed_books_handler(
    app_state: web::Data<AppState>,
    session: Session,
    params: web::Query<PageParams>,
) -> Result<HttpResponse, Error> {
    let template = &app_state.templates;
    let conn = &app_state.conn;
    let page = params.page.unwrap_or(1);
    let number_per_page = params.number_per_page.unwrap_or(DEFAULT_NUMBER_PER_PAGE);
    let (borrowed_books, num_pages) =
        Query::find_borrowed_books_detail_in_page(conn, page, number_per_page).await?;
    let is_admin = is_admin(&session)?;
    let mut ctx = basic_context(&session)?;
    ctx.insert("title", "借阅列表");
    ctx.insert("borrowed_books", &borrowed_books);
    ctx.insert("page", &page);
    ctx.insert("num_pages", &num_pages);
    ctx.insert("number_per_page", &number_per_page);
    ctx.insert("is_admin", &is_admin);
    let body = template.read().unwrap().render("borrow/list.html.tera", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
