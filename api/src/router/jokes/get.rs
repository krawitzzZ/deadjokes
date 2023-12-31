use actix_web::web;
use futures::TryFutureExt;
use uuid::Uuid;

use crate::dto::JokeDto;
use crate::query::*;
use crate::response::*;

use super::get_joke_service;

#[tracing::instrument(name = "api:router:jokes:get_all", skip(state))]
pub async fn get_all(
    state: web::Data<app::AppState>,
    query: web::Query<JokeQuery>,
) -> Response<PaginatedResponse<JokeDto>> {
    let result = get_joke_service(&state)?
        .query(query.into_inner().into())
        .map_err(|e| {
            tracing::warn!(error = %e, "failed to query jokes");
            Error::InternalError
        })
        .await?;

    ok(result.into())
}

#[tracing::instrument(name = "api:router:jokes:find_by_id", skip(state))]
pub async fn find_by_id(
    state: web::Data<app::AppState>,
    path: web::Path<(Uuid,)>,
) -> Response<JokeDto> {
    let (id,) = path.into_inner();
    let joke_opt = get_joke_service(&state)?
        .find_by_id(id)
        .map_err(|e| {
            tracing::warn!(error = %e, joke.id = %id, "failed to fetch a joke by id");
            Error::InternalError
        })
        .await?;

    match joke_opt {
        Some(joke) => ok(joke.into()),
        None => not_found(),
    }
}

#[tracing::instrument(name = "api:router:jokes:get_random", skip(state))]
pub async fn get_random(state: web::Data<app::AppState>) -> Response<JokeDto> {
    let joke = get_joke_service(&state)?
        .get_random()
        .map_err(|e| {
            tracing::warn!(error = %e, "failed to get random joke");
            Error::InternalError
        })
        .await?;

    ok(joke.into())
}
