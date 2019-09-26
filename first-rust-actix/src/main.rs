extern crate askama; 
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use askama::Template;

#[derive(Template)] 
#[template(path = "hello.html")]               
struct HelloTemplate<'a> { 
    name: &'a str, 
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn greeter(s : web::Path<String>) -> impl Responder{
    HelloTemplate{name : s.as_ref()}.render().unwrap()
}

fn main() {
    let ip_addr = std::env::args().nth(1).expect("Specify host. Example: first-rust-actix localhost:8080");
    println!("Will bind to {}", ip_addr);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/{name}", web::get().to(greeter))
    })
    .bind(ip_addr)
    .unwrap()
    .run()
    .unwrap();
}