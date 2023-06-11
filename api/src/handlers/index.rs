use actix_session::Session;
use actix_web::{HttpResponse, web};

use crate::{error::Error, AppState};

use super::basic_context;

pub async fn index_handler(
    app_state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let template = &app_state.templates;
    let mut ctx = basic_context(&session)?;
    ctx.insert("title", "主页");
    ctx.insert("no_title", &true);
    let body = template.read().unwrap().render("index.html.tera", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
