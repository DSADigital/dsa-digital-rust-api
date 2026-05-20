use actix_web::{HttpResponse, Responder, post};

#[post("/user/login")]
async fn user_login() -> impl Responder {
    HttpResponse::Ok().body("user login")
}
