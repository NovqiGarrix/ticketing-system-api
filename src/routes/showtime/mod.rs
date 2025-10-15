mod showtime_routes;

use crate::routes::showtime::showtime_routes::get_showtime_handler;
use actix_web::web::{scope, ServiceConfig};

pub fn showtime_routes(config: &mut ServiceConfig) {
    config.service(scope("/showtime").service(get_showtime_handler));
}
