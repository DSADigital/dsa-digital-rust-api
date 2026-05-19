mod get_all_users;

use actix_web::web::{ServiceConfig, scope};

pub fn routes(config: &mut ServiceConfig) {
    let scope = scope("/api").service(get_all_users::get_all_users);

    config.service(scope);
}
