use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn main() {
    let ip_addr = std::env::args().nth(1).expect("no pattern given");
    println!("Will bind to {}", ip_addr);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind(ip_addr)
    .unwrap()
    .run()
    .unwrap();
}