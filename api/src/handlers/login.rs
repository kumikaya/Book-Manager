use book_manager_service::Query;
use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::{error::Error, AppState, flash_success, flash_error};

use super::basic_context;

pub async fn login_handler(
    app_state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let template = &app_state.templates;
    let mut ctx = basic_context(&session)?;
    ctx.insert("title", "Login");
    let body = template.read().unwrap().render("login.html.tera", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

impl LoginForm {
    pub fn trim(self) -> Self {
        let LoginForm { username, password } = self;
        let username = username.trim().to_string();
        let password = password.trim().to_string();
        Self { username, password }
    }
}

pub async fn login_post_handler(
    app_state: web::Data<AppState>,
    session: Session,
    post_form: web::Form<LoginForm>,
) -> Result<HttpResponse, Error> {
    let LoginForm { username, password } = post_form.into_inner().trim();
    let conn = &app_state.conn;
    if let Some(user) = Query::find_user_by_name(conn, &username).await? {
        if bcrypt::verify(&password, &user.password_hash)? {
            session.insert("user_id", user.id)?;
            session.insert("user_name", user.name)?;
            session.insert("user_nickname", user.nickname)?;
            session.insert("user_permission", user.permission)?;
            flash_success(&session, "登录成功")?;
            return Ok(HttpResponse::Found()
                .append_header(("Location", "/"))
                .finish());
        }
    }

    flash_error(&session, "错误的用户名或密码")?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/login"))
        .finish())
}
