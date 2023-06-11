use book_manager_service::Mutation;
use actix_session::Session;
use actix_web::{web, HttpResponse};
use entity::books;

use crate::{error::Error, AppState, handlers::basic_context, flash_success};

pub async fn new_book_handler(
    app_state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let template = &app_state.templates;
    let mut ctx = basic_context(&session)?;
    ctx.insert("title", "新建图书");
    let body = template.read().unwrap().render("books/new.html.tera", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

pub async fn new_book_post_handler(
    app_state: web::Data<AppState>,
    session: Session,
    post_form: web::Form<books::Model>,
) -> Result<HttpResponse, Error> {
    let post_form = post_form.into_inner();
    let conn = &app_state.conn;
    Mutation::create_book(conn, post_form).await?;
    flash_success(&session, "添加成功")?;
    Ok(HttpResponse::Found()
        .append_header(("Location", "/books"))
        .finish())
}
