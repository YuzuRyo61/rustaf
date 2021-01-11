extern crate env_logger;
use actix_web::{HttpServer, App as AWApp, web};
use actix_web::middleware::Logger;
use rustaf::config::get_config;
use rustaf::server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let cfg = get_config("rustaf_config.toml".to_owned()).unwrap();

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        AWApp::new()
            .route("/", web::get().to(server::index))
            .route("/messages/", web::get().to(server::list_message))
            .route("/messages/", web::post().to(server::post_message))
            .wrap(Logger::default())
    })
        .bind(format!("{}:{}", cfg.server.address, cfg.server.port))?
        .run()
        .await
}
