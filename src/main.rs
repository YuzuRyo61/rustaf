#[macro_use]
extern crate clap;
extern crate env_logger;

use clap::{App, Arg};
use std::process;
use actix_web::{HttpServer, App as AWApp, web};
use actix_web::middleware::Logger;
use rustaf::config::{read_file, Config};
use rustaf::server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true));

    let matches = app.get_matches();
    let cfg_text = match read_file(
        matches
            .value_of("config")
            .unwrap_or("rustaf_config.toml")
            .to_owned()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR: Failed to read file: {}", e);
            process::exit(1);
        }
    };
    let cfg_raw: Result<Config, toml::de::Error> = toml::from_str(&cfg_text);
    let cfg = match cfg_raw {
        Ok(cfg_parsed) => cfg_parsed,
        Err(e) => panic!("Failed parse toml: {}", e)
    };

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        AWApp::new()
            .route("/", web::get().to(server::index))
            .wrap(Logger::default())
    })
        .bind(format!("{}:{}", cfg.server.address, cfg.server.port))?
        .run()
        .await
}
