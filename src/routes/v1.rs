use crate::routes::theaters::theaters_routes;
use actix_web::web::ServiceConfig;

pub fn v1_routes(config: &mut ServiceConfig) {
    config.configure(theaters_routes);
}
