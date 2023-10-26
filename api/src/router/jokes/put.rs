use actix_web::web;
use futures::TryFutureExt;

use app::*;
use uuid::Uuid;

use crate::{dto::*, response::*};

use super::get_joke_service;

#[tracing::instrument(name = "api:router:jokes:update", skip(state, payload))]
pub async fn update(
    state: web::Data<app::AppState>,
    payload: web::Json<UpdateJokeDto>,
    path: web::Path<(Uuid,)>,
) -> Response<JokeDto> {
    let (id,) = path.into_inner();
    let joke = get_joke_service(&state)?
        .update(payload.into_inner().into_data(id))
        .map_err(|e| match e {
            AppError::NotFound(_) => Error::NotFound,
            _ => {
                log::warn!("failed to update a joke with id '{id}': {e}");
                Error::InternalError
            }
        })
        .await?;

    created(joke.into())
}
