extern crate askama; 
use actix_web::{web, App, HttpServer};
mod req_handlers;
mod app_cfg;

const DEFAULT_CONFIG_PATH : &str = "app_cfg.json";

fn main() {
    // let tmp = app_cfg::AppCfg{
    //     resource_reciever : app_cfg::ResourceRecieverType::ActixClient,
    //     inet_addr : String::from("localhost:8087")
    // };

    // println!("Cfg: {}", serde_json::to_string(&tmp).unwrap());

    let cfg_file_specified_path = match std::env::args().nth(1) {
        Some(path) => {
            println!("Will try to read config from specified file {}", path);
            path
        },
        _ => {
            println!("Configuration file wasn't specified. Will try to use default config path {}", 
                DEFAULT_CONFIG_PATH);
            String::from(DEFAULT_CONFIG_PATH)
        }
    };

    let mut cfg_file = std::fs::File::open(cfg_file_specified_path).unwrap();
    let cfg = app_cfg::get_app_config_from_json_file(&mut cfg_file).unwrap();

    let ip_addr = cfg.inet_addr;
    println!("Will bind to {}", ip_addr);

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/proxy_api").configure(req_handlers::configure_proxy_handlers))
    })
    .bind(ip_addr)
    .unwrap()
    .run()
    .unwrap();
}