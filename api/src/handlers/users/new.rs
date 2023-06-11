use book_manager_service::Mutation;
use actix_session::Session;
use actix_web::{web, HttpResponse};
use entity::AccessPermission;
use serde::Deserialize;

use crate::{error::Error, AppState, handlers::basic_context, flash_success, flash_error};

pub async fn register_handler(
    app_state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let template = &app_state.templates;
    let mut ctx = basic_context(&session)?;
    ctx.insert("title", "注册");
    let body = template.read().unwrap().render("users/register.html.tera", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[derive(Debug, Deserialize)]
pub struct RegisterForm {
    username: String,
    nickname: String,
    password: String,
}

impl RegisterForm {
    pub fn trim(self) -> Self {
        let RegisterForm { username, nickname, password } = self;
        let username = username.trim().to_owned();
        let nickname = nickname.trim().to_owned();
        let password = password.trim().to_owned();
        Self { username, nickname, password }
    }
}

// 确保注册信息符合要求
fn verify_register_info(info: &RegisterForm) -> Result<(), String> {
    let RegisterForm { username, nickname, password } = info;
    if username.len() < 3 || nickname.len() < 3 {
        return Err("用户名或昵称至少需要3个字符".to_owned());
    }
    if username.len() > 20 || nickname.len() > 20 {
        return Err("用户名或昵称不能超过20个字符".to_owned());
    }
    if password.len() < 6 {
        return Err("密码至少需要6个字符".to_owned());
    }
    if password.len() > 50 {
        return Err("密码不能超过50个字符".to_owned());
    }
    Ok(())
}

pub async fn register_post_handler(
    app_state: web::Data<AppState>,
    session: Session,
    post_form: web::Form<RegisterForm>,
) -> Result<HttpResponse, Error> {
    let post_form = post_form.into_inner().trim();
    if let Err(msg) = verify_register_info(&post_form) {
        flash_error(&session, msg)?;
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/register"))
            .finish());
    }
    let RegisterForm { username, nickname, password } = post_form;
    let password_hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST)?;
    let conn = &app_state.conn;
    // Insert new user
    Mutation::create_user(conn, username, nickname, password_hash, AccessPermission::User).await?;
    flash_success(&session, "注册成功")?;
    Ok(HttpResponse::Found()
        .append_header(("Location", "/login"))
        .finish())
}
