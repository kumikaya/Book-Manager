use book_manager_service::Mutation;
use actix_web::{web, HttpResponse};

use crate::{error::Error, AppState, handlers::DeleteParams};

pub async fn delete_user_handler(
    app_state: web::Data<AppState>,
    user_id: web::Path<i32>,
    params: web::Query<DeleteParams>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    let source = params.into_inner().source.unwrap_or("/users".to_string());
    let conn = &app_state.conn;
    Mutation::delete_user(conn, user_id).await?;
    Ok(HttpResponse::Found()
        .append_header(("Location", source))
        .finish())
}
