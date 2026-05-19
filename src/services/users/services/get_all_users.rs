use crate::AppState;
use actix_web::{HttpResponse, Responder, get, web::Data};
use serde_json::json;

use crate::services::users::{
    models::UserModel,
    sql_commands::SELECT_ALL_USERS,
};

#[get("/users")]
async fn get_all_users(db: Data<AppState>) -> impl Responder {
    let result = sqlx::query_as::<_, UserModel>(SELECT_ALL_USERS)
        .fetch_all(&db.client_db)
        .await;

    match result {
        Ok(users) => {
            let user_list = users
                .iter()
                .map(|user| {
                    json!({
                        "id": user.id,
                        "first_name": user.first_name,
                        "role": user.role,
                        "created_at": user.created_at,
                    })
                })
                .collect::<Vec<serde_json::Value>>();

            HttpResponse::Ok().json(json!({
                "status": "ok",
                "users": user_list,
                "size": users.len(),
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("{}", e)
        })),
    }
}
