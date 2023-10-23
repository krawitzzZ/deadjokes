use shaku::{HasProvider, Module, Provider};
use uuid::Uuid;

use crate::{AppError, AppResult, Joke, JokeQuery, JokeRepository, QueryResult};

use super::{CreateJokeData, UpdateJokeData};

pub struct JokeService {
    repo: Box<dyn JokeRepository>,
}

impl JokeService {
    #[tracing::instrument(name = "app:joke:service:get_random", skip(self))]
    pub async fn get_random(&self) -> AppResult<Joke> {
        self.repo.get_random().await
    }

    #[tracing::instrument(name = "app:joke:service:find_by_id", skip(self), fields(joke.id = %id))]
    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Joke>> {
        self.repo.find_by_id(id).await
    }

    #[tracing::instrument(name = "app:joke:service:query", skip(self))]
    pub async fn query(&self, query: JokeQuery) -> AppResult<QueryResult<Joke>> {
        self.repo.query(query).await
    }

    #[tracing::instrument(name = "app:joke:service:create", skip(self, data))]
    pub async fn create(&self, data: CreateJokeData) -> AppResult<Joke> {
        self.repo.create(data).await
    }

    #[tracing::instrument(name = "app:joke:service:update", skip(self), fields(joke.id = %data.id))]
    pub async fn update(&self, data: UpdateJokeData) -> AppResult<Joke> {
        let mut joke = self
            .repo
            .find_by_id(data.id)
            .await?
            .ok_or(AppError::NotFound(format!(
                "joke with id {} not found",
                data.id
            )))?;

        joke.body = data.body;

        self.repo.update(joke).await
    }
}

impl<M: Module + HasProvider<dyn JokeRepository>> Provider<M> for JokeService {
    type Interface = JokeService;

    fn provide(module: &M) -> Result<Box<JokeService>, Box<dyn std::error::Error + 'static>> {
        let repo: Box<dyn JokeRepository> = module
            .provide()
            .map_err(|e| AppError::Unexpected(anyhow::anyhow!(e.to_string())))?;
        Ok(Box::new(JokeService { repo }))
    }
}
