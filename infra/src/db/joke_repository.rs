use futures_util::TryFutureExt;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseBackend, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, QueryTrait, Statement,
};
use sea_query::{extension::postgres::PgExpr, Expr, Func, Order, PgFunc};
use std::sync::Arc;
use uuid::Uuid;

use app::{
    joke::{CreateJokeData, Joke},
    AppError, AppResult, DbPool, JokeQuery, JokeRepository, ModelRepository, QueryResult,
};
use entity::*;

use super::{map_err, PAGE_SIZE};

#[derive(shaku::Provider)]
#[shaku(interface = JokeRepository)]
pub struct PostgresJokeRepository {
    #[shaku(inject)]
    pool: Arc<dyn DbPool>,
}

impl PostgresJokeRepository {
    #[tracing::instrument(name = "infra:db:joke_repo:get_full_text_config_oid", skip_all)]
    async fn get_full_text_config_oid(&self) -> AppResult<Option<u32>> {
        let conn = self.pool.get();
        let stm = "SELECT oid, cfgname FROM pg_ts_config WHERE cfgname = 'english';";
        let query_res = conn
            .query_one(Statement::from_string(DatabaseBackend::Postgres, stm))
            .map_err(map_err::into_app_db_err)
            .await?;
        let oid: Option<u32> = query_res
            .ok_or(AppError::Internal(
                "failed to get full text config oid".into(),
            ))?
            .try_get("", "oid")
            .map_err(map_err::into_app_db_err)?;

        Ok(oid)
    }

    #[tracing::instrument(name = "infra:db:joke_repo:find_by_body", skip(self))]
    async fn find_by_body(&self, body: &str) -> AppResult<Option<Joke>> {
        let conn = self.pool.get();
        let oid = self.get_full_text_config_oid().await?;
        let filter = Expr::expr(PgFunc::to_tsvector(Expr::col(JokeColumn::Body), oid))
            .matches(PgFunc::phraseto_tsquery(body, oid));

        JokeEntity::find()
            .filter(filter)
            .limit(1)
            .one(conn)
            .map_err(map_err::into_app_db_err)
            .and_then(|om| async { Ok(om.map(Into::into)) })
            .await
    }
}

#[async_trait::async_trait]
impl ModelRepository for PostgresJokeRepository {
    type Model = Joke;
    type Query = JokeQuery;
    type CreateData = CreateJokeData;

    #[tracing::instrument(name = "infra:db:joke_repo:query", skip(self))]
    async fn query(&self, query: Self::Query) -> AppResult<QueryResult<Self::Model>> {
        let conn = self.pool.get();
        let oid = if query.body.is_some() {
            self.get_full_text_config_oid().await?
        } else {
            None
        };
        let paginator = JokeEntity::find()
            .apply_if(query.body, |query, body| {
                query.filter(
                    Expr::expr(PgFunc::to_tsvector(Expr::col(JokeColumn::Body), oid))
                        .matches(PgFunc::websearch_to_tsquery(body, oid)),
                )
            })
            .paginate(conn, PAGE_SIZE);
        let pagination_info = paginator
            .num_items_and_pages()
            .map_err(map_err::into_app_db_err)
            .await?;

        let jokes = paginator
            .fetch_page(query.page)
            .map_err(map_err::into_app_db_err)
            .map_ok(|e| e.into_iter().map(JokeModel::into).collect())
            .await?;

        Ok(QueryResult::new(
            query.page,
            pagination_info.number_of_pages,
            pagination_info.number_of_items,
            jokes,
        ))
    }

    #[tracing::instrument(name = "infra:db:joke_repo:find_by_id", skip(self))]
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Self::Model>> {
        JokeEntity::find_by_id(id)
            .one(self.pool.get())
            .map_err(map_err::into_app_db_err)
            .map_ok(|mc| mc.map(JokeModel::into))
            .await
    }

    #[tracing::instrument(name = "infra:db:joke_repo:create", skip(self))]
    async fn create(&self, data: Self::CreateData) -> AppResult<Self::Model> {
        if self.find_by_body(&data.body).await?.is_some() {
            return Err(AppError::Duplicate(format!("similar joke already exists")));
        }

        JokeActiveModel::from_data(data)
            .insert(self.pool.get())
            .await
            .map_err(map_err::into_app_db_err)
            .map(Into::into)
    }

    #[tracing::instrument(name = "infra:db:joke_repo:update", skip(self))]
    async fn update(&self, model: Self::Model) -> AppResult<Self::Model> {
        let conn = self.pool.get();
        let joke_am: JokeActiveModel = JokeModel::from(model).into();

        joke_am
            .reset_all()
            .update(conn)
            .await
            .map_err(map_err::into_app_db_err)
            .map(Into::into)
    }
}

#[async_trait::async_trait]
impl JokeRepository for PostgresJokeRepository {
    #[tracing::instrument(name = "infra:db:joke_repo:get_random", skip(self))]
    async fn get_random(&self) -> AppResult<Self::Model> {
        JokeEntity::find()
            .order_by(Expr::expr(Func::random()), Order::Asc)
            .limit(1)
            .one(self.pool.get())
            .map_err(map_err::into_app_db_err)
            .await?
            .map(JokeModel::into)
            .ok_or(AppError::Internal("failed to get random joke".into()))
    }
}
