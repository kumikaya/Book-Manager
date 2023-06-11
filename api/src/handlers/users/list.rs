use book_manager_service::Query;
use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{error::Error, AppState, handlers::{basic_context, PageParams, DEFAULT_NUMBER_PER_PAGE}};


pub async fn list_users_handler(
    app_state: web::Data<AppState>,
    session: Session,
    params: web::Query<PageParams>,
) -> Result<HttpResponse, Error> {
    if session.get::<i32>("user_id")?.is_some() {
        let template = &app_state.templates;
        let conn = &app_state.conn;
        let page = params.page.unwrap_or(1);
        let number_per_page = params.number_per_page.unwrap_or(DEFAULT_NUMBER_PER_PAGE);
        let (users, num_pages) = Query::find_users_in_page(conn, page, number_per_page).await?;
        let mut ctx = basic_context(&session)?;
        ctx.insert("title", "用户列表");
        ctx.insert("users", &users);
        ctx.insert("page", &page);
        ctx.insert("num_pages", &num_pages);
        ctx.insert("number_per_page", &number_per_page);
        let body = template.read().unwrap().render("users/list.html.tera", &ctx)?;
        Ok(HttpResponse::Ok().content_type("text/html").body(body))
    } else {
        Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish())
    }
}
