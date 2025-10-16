mod movies_routes;
use actix_web::web::{ServiceConfig, scope};
use movies_routes::{get_movie_handler, get_movies_handler};

pub fn movie_routes(config: &mut ServiceConfig) {
    config.service(
        scope("/movies")
            .service(get_movies_handler)
            .service(get_movie_handler),
    );
}
