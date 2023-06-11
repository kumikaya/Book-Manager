use actix_session::Session;
use actix_web::HttpResponse;

use crate::error::Error;

pub async fn logout_handler(session: Session) -> Result<HttpResponse, Error> {
    session.clear();
    session.renew();
    Ok(HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish())
}
