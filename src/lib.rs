#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate toml;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub mod config;
pub mod server;
pub mod schema;
pub mod models;

pub fn db_connect() -> PgConnection {
    let cfg = config::get_config("rustaf_config.toml".to_owned()).unwrap();
    PgConnection::establish(&cfg.database.uri)
        .expect(&format!("Error Connecting to {}", &cfg.database.uri))
}
