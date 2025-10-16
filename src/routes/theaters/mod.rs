mod theaters_routes;

use actix_web::web::{ServiceConfig, scope};
use theaters_routes::{get_theater_showtime_handler, get_theaters_handler};

pub fn theaters_routes(config: &mut ServiceConfig) {
    config.service(
        scope("/theaters")
            .service(get_theaters_handler)
            .service(get_theater_showtime_handler),
    );
}
