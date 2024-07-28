use std::rc::Rc;

use actix_web::Error;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::HeaderValue,
    HttpMessage, HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};

use crate::utility::token::decode_jwt;

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .and_then(|hv: &HeaderValue| hv.to_str().ok())
            .map(|s: &str| s.trim_start_matches("Bearer "))
            .map(String::from);

        if let Some(token) = auth_header {
            match decode_jwt(token) {
                Ok(token_data) => {
                    req.extensions_mut().insert(token_data.claims.sub);
                    let fut = self.service.call(req);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res.map_into_left_body())
                    })
                }
                Err(_) => Box::pin(async {
                    let (req, _pl) = req.into_parts();
                    let res = HttpResponse::Unauthorized().finish().map_into_right_body();
                    Ok(ServiceResponse::new(req, res))
                }),
            }
        } else {
            Box::pin(async {
                let (req, _pl) = req.into_parts();
                let res = HttpResponse::Unauthorized().finish().map_into_right_body();
                Ok(ServiceResponse::new(req, res))
            })
        }
    }
}
