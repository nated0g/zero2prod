use actix_web::{web::Form, HttpResponse, Responder};
use serde::Deserialize;

pub async fn subscribe(_form: Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
