mod movies;
mod showtime;
mod theaters;
mod v1;

use actix_web::web::{ServiceConfig, scope};

pub fn routes(config: &mut ServiceConfig) {
    config.service(scope("/v1").configure(v1::v1_routes));
}
