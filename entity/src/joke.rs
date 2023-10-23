use sea_orm::DeriveEntityModel;
use sea_orm::{entity::prelude::*, Set};
use uuid::Uuid;

use app::joke::{CreateJokeData, Joke};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "jokes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Joke> for Model {
    fn from(joke: Joke) -> Model {
        Model {
            id: joke.id,
            body: joke.body,
            created_at: joke.created_at,
            last_updated_at: joke.last_updated_at,
        }
    }
}

impl Into<Joke> for Model {
    fn into(self) -> Joke {
        Joke {
            id: self.id,
            body: self.body,
            created_at: self.created_at,
            last_updated_at: self.last_updated_at,
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModel {
    pub fn from_data(data: CreateJokeData) -> Self {
        let mut am = <Self as ActiveModelBehavior>::new();
        am.body = Set(data.body);
        am
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let mut active_model = <Self as sea_orm::ActiveModelTrait>::default();
        active_model.id = Set(Uuid::new_v4());
        active_model
    }

    async fn before_save<C: ConnectionTrait>(
        mut self,
        _db: &C,
        insert: bool,
    ) -> Result<Self, DbErr> {
        if !insert {
            self.last_updated_at = Set(chrono::offset::Utc::now());
        }
        Ok(self)
    }
}
