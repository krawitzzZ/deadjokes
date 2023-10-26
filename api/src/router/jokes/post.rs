use actix_web::web;
use futures::TryFutureExt;

use app::*;

use crate::{dto::*, response::*};

use super::get_joke_service;

#[tracing::instrument(name = "api:router:jokes:create", skip(state, payload))]
pub async fn create(
    state: web::Data<app::AppState>,
    payload: web::Json<CreateJokeDto>,
) -> Response<JokeDto> {
    let joke = get_joke_service(&state)?
        .create(payload.into_inner().into_data())
        .map_err(|e| match e {
            AppError::Duplicate(_) => Error::Conflict,
            _ => {
                log::warn!("failed to create a joke: {e}");
                Error::InternalError
            }
        })
        .await?;

    created(joke.into())
}
