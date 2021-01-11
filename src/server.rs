use actix_web::{Responder, HttpResponse, web};

use crate::db_connect;
use crate::models::*;
use super::diesel::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct ErrResponse {
    message: String
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("It works!")
}

pub async fn list_message() -> impl Responder {
    use crate::schema::messages::dsl::*;

    let conn = db_connect();
    let data = messages
        .limit(30)
        .load::<Messages>(&conn)
        .expect("Error loading messages");

    HttpResponse::Ok().json(data)
}

pub async fn post_message(body: web::Json<CreateMessages>) -> impl Responder {
    use crate::schema::messages;
    let conn = db_connect();
    let res: Messages = diesel::insert_into(messages::table)
        .values(&*body)
        .get_result(&conn)
        .expect("Error saving new post");
    HttpResponse::Ok().json(res)
}
