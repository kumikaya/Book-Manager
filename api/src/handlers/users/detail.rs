use book_manager_service::Query;
use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{
    error::Error,
    handlers::{basic_context, is_admin},
    AppState,
};

pub async fn user_detail_handler(
    app_state: web::Data<AppState>,
    session: Session,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let template = &app_state.templates;
    let user_id = user_id.into_inner();
    let conn = &app_state.conn;
    let cache_id = session
        .get::<i32>("user_id")?
        .ok_or(Error::user_not_found())?;
    let is_admin = is_admin(&session)?;
    if cache_id != user_id && !is_admin {
        return Err(Error::unauthorized());
    }
    let user = Query::find_user_by_id(conn, user_id)
        .await?
        .ok_or(Error::user_not_found())?;
    let borrowed_books_info = Query::find_borrowed_books_detail_by_user_id(conn, user_id).await?;
    let mut ctx = basic_context(&session)?;
    ctx.insert("title", "用户详情");
    ctx.insert("user", &user);
    ctx.insert("borrowed_books_info", &borrowed_books_info);
    let body = template.read().unwrap().render("users/detail.html.tera", &ctx).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
