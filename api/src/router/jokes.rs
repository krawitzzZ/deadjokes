mod get;
mod post;
mod put;

use actix_web::web;

use crate::response::*;

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("")
            .name("query_and_create_jokes")
            .route(web::get().to(get::get_all))
            .route(web::post().to(post::create)),
    );
    config.service(
        web::resource("random")
            .name("get_random_joke")
            .route(web::get().to(get::get_random)),
    );
    config.service(
        web::resource("{id}")
            .name("joke_details")
            .route(web::get().to(get::find_by_id))
            .route(web::put().to(put::update)),
    );
}

fn get_joke_service(
    state: &web::Data<app::AppState>,
) -> Result<Box<app::joke::JokeService>, Error> {
    state.container().provide().map_err(|e| {
        tracing::error!(
            error = %e,
            "failed to retrieve JokeService"
        );
        Error::InternalError
    })
}
