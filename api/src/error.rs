use book_manager_service::sea_orm;
use actix_web::{HttpResponse, ResponseError};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    DbErr(sea_orm::error::DbErr),
    ActixError(actix_web::Error),
    TeraError(tera::Error),
    SessionGetError(actix_session::SessionGetError),
    SessionInsertError(actix_session::SessionInsertError),
    BcryptError(bcrypt::BcryptError),
    Other(String),
}

impl Error {
    pub fn new<T: ToString>(msg: T) -> Self {
        Error::Other(msg.to_string())
    }

    pub fn unlogin() -> Self {
        Error::ActixError(actix_web::error::ErrorUnauthorized("unlogin"))
    }

    pub fn unauthorized() -> Self {
        Error::ActixError(actix_web::error::ErrorUnauthorized("unauthorized"))
    }

    pub fn user_not_found() -> Self {
        Error::ActixError(actix_web::error::ErrorNotFound("User not found"))
    }

    pub fn book_not_found() -> Self {
        Error::ActixError(actix_web::error::ErrorNotFound("Book not found"))
    }
    pub fn borrow_record_not_found() -> Self {
        Error::ActixError(actix_web::error::ErrorNotFound("Borrow record not found"))
    }

    pub fn email_not_found() -> Self {
        Error::ActixError(actix_web::error::ErrorNotFound("Email not found"))
    }

    pub fn bad_request<T: ToString>(msg: T) -> Self {
        Error::ActixError(actix_web::error::ErrorBadRequest(msg.to_string()))
    }
}

impl std::error::Error for Error {}

impl From<sea_orm::error::DbErr> for Error {
    fn from(err: sea_orm::error::DbErr) -> Self {
        Error::DbErr(err)
    }
}

impl From<actix_web::Error> for Error {
    fn from(err: actix_web::Error) -> Self {
        Error::ActixError(err)
    }
}

impl From<tera::Error> for Error {
    fn from(err: tera::Error) -> Self {
        Error::TeraError(err)
    }
}

impl From<actix_session::SessionGetError> for Error {
    fn from(err: actix_session::SessionGetError) -> Self {
        Error::SessionGetError(err)
    }
}

impl From<actix_session::SessionInsertError> for Error {
    fn from(err: actix_session::SessionInsertError) -> Self {
        Error::SessionInsertError(err)
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(err: bcrypt::BcryptError) -> Self {
        Error::BcryptError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::DbErr(err) => write!(f, "DbErr: {}", err),
            Error::ActixError(err) => write!(f, "ActixError: {}", err),
            Error::TeraError(err) => write!(f, "TeraError: {}", err),
            Error::SessionGetError(err) => write!(f, "SessionGetError: {}", err),
            Error::SessionInsertError(err) => write!(f, "SessionInsertError: {}", err),
            Error::BcryptError(err) => write!(f, "BcryptError: {}", err),
            Error::Other(msg) => write!(f, "Other: {}", msg),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let mut builder = HttpResponse::InternalServerError();
        match self {
            Error::DbErr(err) => builder.body(err.to_string()),
            Error::ActixError(err) => builder.body(err.to_string()),
            Error::TeraError(err) => builder.body(err.to_string()),
            Error::SessionGetError(err) => builder.body(err.to_string()),
            Error::SessionInsertError(err) => builder.body(err.to_string()),
            Error::BcryptError(err) => builder.body(err.to_string()),
            Error::Other(msg) => builder.body(msg.to_string()),
        }
    }
}
