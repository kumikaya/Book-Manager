use actix_session::Session;
use actix_web::HttpResponse;

use crate::error::Error;


pub async fn background_handler(session: Session) -> Result<HttpResponse, Error> {
    if let Some(switch) = session.get::<u32>("background")? {
        session.insert("background", &(switch + 1))?;
    } else {
        session.insert("background", 1u32)?;
    }
    Ok(HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish())
}
