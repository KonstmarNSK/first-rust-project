use actix_web::web;
mod proxy;

/// adds proxy methods to specified configuration
pub fn configure_proxy_handlers(config : &mut web::ServiceConfig){
    config.service(proxy::proxy);
}