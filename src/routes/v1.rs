use crate::routes::showtime::showtime_routes;
use crate::routes::theaters::theaters_routes;
use actix_web::web::ServiceConfig;

pub fn v1_routes(config: &mut ServiceConfig) {
    config.configure(theaters_routes).configure(showtime_routes);
}
