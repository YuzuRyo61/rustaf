use actix_web::{Responder, HttpResponse};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("It works!")
}
