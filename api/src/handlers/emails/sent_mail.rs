use book_manager_service::Query;
use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::{error::Error, handlers::{basic_context, DEFAULT_NUMBER_PER_PAGE}, AppState};

#[derive(Debug, Deserialize)]
pub struct SentMailParams {
    page: Option<u64>,
    number_per_page: Option<u64>,
}

pub async fn sent_email_handler(
    app_state: web::Data<AppState>,
    session: Session,
    params: web::Query<SentMailParams>,
) -> Result<HttpResponse, Error> {
    let user_id = session
        .get::<i32>("user_id")?
        .ok_or(Error::user_not_found())?;
    let template = &app_state.templates;
    let conn = &app_state.conn;
    let page = params.page.unwrap_or(1);
    let number_per_page = params.number_per_page.unwrap_or(DEFAULT_NUMBER_PER_PAGE);
    let (sent_emails, num_pages) =
        Query::find_emails_in_page_by_sender_id(conn, user_id, page, number_per_page).await?;
    let mut ctx = basic_context(&session)?;
    ctx.insert("title", "发件箱");
    ctx.insert("sent_emails", &sent_emails);
    ctx.insert("page", &page);
    ctx.insert("num_pages", &num_pages);
    ctx.insert("number_per_page", &number_per_page);
    let body = template.read().unwrap().render("emails/sent_mail.html.tera", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
