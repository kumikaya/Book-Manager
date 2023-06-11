use actix_session::Session;
use actix_web::{web, HttpResponse};
use book_manager_service::Query;
use serde::Deserialize;

use crate::{error::Error, AppState, flash_error};

use super::{basic_context, PageParams, DEFAULT_NUMBER_PER_PAGE};

const MIN_KEYWORD_LENGTH: usize = 3;

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    keyword: String,
    search_type: String,
}

pub async fn search_handler(
    app_state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let template = &app_state.templates;
    let mut ctx = basic_context(&session)?;
    ctx.insert("title", "搜索");
    let body = template.read().unwrap().render("search.html.tera", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

pub async fn search_get_handler(
    app_state: web::Data<AppState>,
    session: Session,
    search_params: web::Query<SearchParams>,
    page_params: web::Query<PageParams>,
) -> Result<HttpResponse, Error> {
    let template = &app_state.templates;
    let conn = &app_state.conn;
    let page = page_params.page.unwrap_or(1);
    let SearchParams {
        keyword,
        search_type,
    } = search_params.into_inner();
    let number_per_page = page_params
    .number_per_page
    .unwrap_or(DEFAULT_NUMBER_PER_PAGE);
    let mut ctx = basic_context(&session)?;
    if keyword.len() < MIN_KEYWORD_LENGTH {
        let msg = if keyword.is_empty() {
            "搜索内容不能为空"
        } else {
            "搜索内容过短"
        };
        flash_error(&session, msg)?;
        return Ok(HttpResponse::Found().append_header(("Location", "/search")).finish());
    }
    ctx.insert("no_title", &true);
    ctx.insert("result_type", &search_type);
    ctx.insert("page", &page);
    ctx.insert("number_per_page", &number_per_page);
    let body = match search_type.as_str() {
        "users" => {
            let (users, num_pages) =
                Query::find_users_by_keyword_in_page(conn, &keyword, page, number_per_page).await?;
            ctx.insert("title", "用户搜索结果");
            ctx.insert("users", &users);
            ctx.insert("num_pages", &num_pages);
            template.read().unwrap().render("users/list.html.tera", &ctx)?
        },
        "books" => {
            let (books, num_pages) =
                Query::find_books_by_keyword_in_page(conn, &keyword, page, number_per_page).await?;
            ctx.insert("title", "图书搜索结果");
            ctx.insert("books", &books);
            ctx.insert("num_pages", &num_pages);
            template.read().unwrap().render("books/list.html.tera", &ctx)?
        },
        _ => return Err(Error::bad_request("Invalid search type")),
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
