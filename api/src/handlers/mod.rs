use actix_session::Session;
use actix_web::{error, web, HttpRequest, HttpResponse};
use entity::AccessPermission;
use serde::Deserialize;

use crate::{error::Error, AppState};

pub mod books;
pub mod borrow;
pub mod emails;
pub mod index;
pub mod login;
pub mod logout;
pub mod search;
pub mod users;
pub mod background;

const DEFAULT_NUMBER_PER_PAGE: u64 = 8;

#[derive(Debug, Deserialize)]
pub struct DeleteParams {
    source: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PageParams {
    page: Option<u64>,
    number_per_page: Option<u64>,
}

pub async fn not_found(
    data: web::Data<AppState>,
    session: Session,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut ctx = basic_context(&session)?;
    ctx.insert("uri", request.uri().path());

    let template = &data.templates;
    let body = template
        .read()
        .unwrap()
        .render("error/404.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

pub async fn reload_templates(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    data.templates
        .write()
        .unwrap()
        .full_reload()
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Found().append_header(("Location", "/")).finish())
}

fn basic_context(session: &Session) -> Result<tera::Context, Error> {
    let mut ctx = tera::Context::new();
    if let Some(user_id) = session.get::<i32>("user_id")? {
        ctx.insert("user_id", &user_id);
    }
    if let Some(user_name) = session.get::<String>("user_name")? {
        ctx.insert("user_name", &user_name);
    }
    if let Some(user_nickname) = session.get::<String>("user_nickname")? {
        ctx.insert("user_nickname", &user_nickname);
    }
    if let Some(user_permission) = session.get::<AccessPermission>("user_permission")? {
        ctx.insert("user_permission", &user_permission);
    }
    if let Some(flash) = session.get::<crate::FlashData>("flash")? {
        session.remove("flash");
        ctx.insert("flash", &flash);
    }
    if let Some(switch) = session.get::<u32>("background")? {
        ctx.insert("background", &switch);
    }
    Ok(ctx)
}

fn is_admin(session: &Session) -> Result<bool, Error> {
    Ok(session
        .get::<AccessPermission>("user_permission")?
        .unwrap_or(AccessPermission::Guest)
        .is_admin())
}
