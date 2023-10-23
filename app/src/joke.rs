pub mod service;

use uuid::Uuid;

pub use service::*;

#[derive(Debug)]
pub struct Joke {
    pub id: Uuid,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateJokeData {
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct UpdateJokeData {
    pub id: Uuid,
    pub body: String,
}
