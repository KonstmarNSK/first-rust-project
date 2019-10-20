use actix_web::{web, Responder, get};

#[get("/get/{user_token}/{address}")]
pub fn proxy(req_info : web::Path<(String, String)>) -> impl Responder {
    format!("token: {}, address: {}", req_info.0, req_info.1)
}