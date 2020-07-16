use actix_web::{HttpResponse, Responder};
use actix_web::get;
use serde_json::json;

#[get("/")]
pub async fn index() -> impl Responder {
    let contents = json!({
        "name": "Minotaur",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Authentication system for modrinth.",
        "documentation": "https://modrinth.com/developers"
    });
    HttpResponse::Ok().json(contents)
}