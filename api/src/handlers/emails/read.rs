use book_manager_service::Query;
use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{error::Error, handlers::basic_context, AppState};

use super::{get_email_access, EmailAccess};

pub async fn email_detail_handler(
    app_state: web::Data<AppState>,
    session: Session,
    email_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let template = &app_state.templates;
    let email_id = email_id.into_inner();
    let conn = &app_state.conn;
    let user_id = session
        .get::<i32>("user_id")?
        .ok_or(Error::user_not_found())?;
    let email_detail = Query::find_email_detail_by_id(conn, email_id)
        .await?
        .ok_or(Error::email_not_found())?;
    let access = get_email_access(&email_detail.clone().into(), user_id);
    if !matches!(access, EmailAccess::Unrelated) {
        let mut ctx = basic_context(&session)?;
        ctx.insert("title", "邮件内容");
        ctx.insert("email_detail", &email_detail);
        let body = template.read().unwrap().render("emails/read.html.tera", &ctx).unwrap();
        Ok(HttpResponse::Ok().content_type("text/html").body(body))
    } else {
        return Err(Error::unauthorized());
    }
}
