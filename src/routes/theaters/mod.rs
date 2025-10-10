mod theaters_routes;

use actix_web::web::{scope, ServiceConfig};
use theaters_routes::get_theaters_handler;

pub fn theaters_routes(config: &mut ServiceConfig) {
    config.service(scope("/theaters").service(get_theaters_handler));
}
