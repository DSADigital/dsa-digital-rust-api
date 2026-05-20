use actix_web::{HttpResponse, Responder, post};

#[post("/user/register")]
pub async fn create_new_user() -> impl Responder {
    HttpResponse::Ok().body("user created")
}
