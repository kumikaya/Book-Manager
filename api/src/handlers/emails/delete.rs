use book_manager_service::{Mutation, Query};
use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{error::Error, AppState, handlers::DeleteParams, flash_error};

use super::get_email_access;

pub async fn delete_email_handler(
    app_state: web::Data<AppState>,
    session: Session,
    email_id: web::Path<i32>,
    params: web::Query<DeleteParams>,
) -> Result<HttpResponse, Error> {
    let user_id = session
        .get::<i32>("user_id")?
        .ok_or(Error::unauthorized())?;
    let email_id = email_id.into_inner();
    let source = params.into_inner().source.unwrap_or("/emails".to_string());
    let conn = &app_state.conn;
    let email = Query::find_email_by_id(conn, email_id)
        .await?
        .ok_or(Error::email_not_found())?;
    let access = get_email_access(&email, user_id);
    match access {
        super::EmailAccess::SenderAndRecipient => {
            Mutation::delete_email(conn, email_id).await?;
        }
        super::EmailAccess::Sender => {
            Mutation::delete_email_by_id_on_sender(conn, email_id).await?;
        }
        super::EmailAccess::Recipient => {
            Mutation::delete_email_by_id_on_recipient(conn, email_id).await?;
        }
        super::EmailAccess::Unrelated => {
            flash_error(&session, "您没有权限删除该邮件")?;
        }
    }
    Ok(HttpResponse::Found()
        .append_header(("Location", source))
        .finish())
}
