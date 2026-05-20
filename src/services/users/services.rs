mod create_new_user;
mod get_all_users;
mod user_login;

use actix_web::web::{ServiceConfig, scope};

pub fn routes(config: &mut ServiceConfig) {
    let scope = scope("/api")
        .service(get_all_users::get_all_users)
        .service(create_new_user::create_new_user)
        .service(user_login::user_login);

    config.service(scope);
}
