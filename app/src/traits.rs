use sea_orm::DbConn;
use shaku::Interface;
use uuid::Uuid;

use crate::*;

pub trait DbPool: Interface {
    fn get(&self) -> &DbConn;
}

#[async_trait::async_trait]
pub trait ModelRepository: Interface {
    type Model;
    type Query;
    type CreateData;

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Self::Model>>;
    async fn query(&self, query: Self::Query) -> AppResult<QueryResult<Self::Model>>;
    async fn create(&self, model_data: Self::CreateData) -> AppResult<Self::Model>;
    async fn update(&self, model: Self::Model) -> AppResult<Self::Model>;
}

#[async_trait::async_trait]
pub trait JokeRepository:
    ModelRepository<Model = Joke, Query = JokeQuery, CreateData = CreateJokeData>
{
    async fn get_random(&self) -> AppResult<Self::Model>;
}

#[async_trait::async_trait]
pub trait Assets: Interface {
    async fn initial_jokes_seed(&self) -> AppResult<String>;
}
