use actix_session::Session;
use actix_web::{web, HttpResponse};
use book_manager_service::{Mutation, Query};
use entity::EmailCategory;
use serde::Deserialize;

use crate::{
    error::Error, flash_error, flash_success, handlers::basic_context, AppState,
};

#[derive(Debug, Deserialize)]
pub struct SendParams {
    recipient: Option<String>,
}

pub async fn new_email_handler(
    app_state: web::Data<AppState>,
    session: Session,
    params: web::Query<SendParams>,
) -> Result<HttpResponse, Error> {
    let template = &app_state.templates;
    let mut ctx = basic_context(&session)?;
    ctx.insert("title", "发送邮件");
    if let Some(recipient) = &params.recipient {
        ctx.insert("recipient", recipient.trim());
    }
    let body = template.read().unwrap().render("emails/send.html.tera", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[derive(Debug, Deserialize)]
pub struct SendForm {
    category: EmailCategory,
    recipient: String,
    subject: String,
    content: String,
}

pub async fn new_email_post_handler(
    app_state: web::Data<AppState>,
    session: Session,
    post_form: web::Form<SendForm>,
) -> Result<HttpResponse, Error> {
    let SendForm {
        category,
        recipient,
        subject,
        content,
    } = post_form.into_inner();

    let recipient = recipient.trim();
    let conn = &app_state.conn;

    let sender_id = session
        .get::<i32>("user_id")?
        .ok_or(Error::unauthorized())?;

    let recipient_id_opt = match category {
        EmailCategory::Regular => Query::find_user_by_name(conn, recipient)
            .await?
            .map(|user| user.id),
        _ => Some(1),
    };

    match recipient_id_opt {
        Some(recipient_id) => {
            Mutation::create_email(conn, category, sender_id, recipient_id, subject, content)
                .await?;
            flash_success(&session, "邮件发送成功")?;
        }
        None => flash_error(&session, "用户不存在")?,
    };

    Ok(HttpResponse::Found()
        .append_header(("Location", "send"))
        .finish())
}
