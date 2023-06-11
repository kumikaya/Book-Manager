use book_manager_service::{Mutation, Query};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use entity::books;

use crate::{error::Error, AppState, handlers::basic_context, flash_success};

pub async fn edit_book_handler(
    app_state: web::Data<AppState>,
    session: Session,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let template = &app_state.templates;
    let id = id.into_inner();
    let conn = &app_state.conn;
    let book = Query::find_book_by_id(conn, id)
        .await?
        .ok_or(actix_web::error::ErrorNotFound("Book not found"))?;
    let mut ctx = basic_context(&session)?;
    ctx.insert("title", "编辑图书");
    ctx.insert("book", &book);
    let body = template.read().unwrap().render("books/edit.html.tera", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

pub async fn edit_book_post_handler(
    app_state: web::Data<AppState>,
    session: Session,
    id: web::Path<i32>,
    post_form: web::Form<books::Model>,
) -> Result<HttpResponse, Error> {
    let post_form = post_form.into_inner();
    let id = id.into_inner();
    let conn = &app_state.conn;
    Mutation::update_book_by_id(conn, id, post_form).await?;
    flash_success(&session, "修改成功")?;
    Ok(HttpResponse::Found()
        .append_header(("Location", "/books"))
        .finish())
}
