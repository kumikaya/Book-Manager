use std::future::{ready, Ready};

use actix_session::{Session, SessionExt};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error,
};
use entity::AccessPermission;
use futures_util::future::LocalBoxFuture;


// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Permission {
    permission: AccessPermission,
}

impl Permission {
    pub fn new(permission: AccessPermission) -> Self {
        Self { permission }
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Permission
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PermissionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PermissionMiddleware {
            service,
            permission: self.permission.clone(),
        }))
    }
}

pub struct PermissionMiddleware<S> {
    permission: AccessPermission,
    service: S,
}

impl<S, B> Service<ServiceRequest> for PermissionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let session = request.get_session();
        let fut = self.service.call(request);
        let permission = self.permission.clone();
        Box::pin(async move {
            if perm_verify(session, permission) {
                fut.await
            } else {
                Err(actix_web::error::ErrorUnauthorized("unauthorized"))
            }
        })
    }
}

fn perm_verify(session: Session, requested_perm: AccessPermission) -> bool {
    if let Some(perm) = session
        .get::<AccessPermission>("user_permission")
        .unwrap_or(None)
    {
        return (perm as u8) <= (requested_perm as u8);
    }
    false
}
