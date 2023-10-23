use uuid::Uuid;

use app::joke::{CreateJokeData, Joke, UpdateJokeData};

/// JSON representation of a Joke.
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JokeDto {
    pub id: Uuid,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Joke> for JokeDto {
    fn from(joke: Joke) -> Self {
        JokeDto {
            id: joke.id,
            body: joke.body,
            created_at: joke.created_at,
            last_updated_at: joke.last_updated_at,
        }
    }
}

/// JSON payload to create a Joke.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateJokeDto {
    pub body: String,
}

impl CreateJokeDto {
    pub fn into_data(self) -> CreateJokeData {
        CreateJokeData {
            body: self.body.trim().to_string(),
        }
    }
}

/// JSON payload to update a Joke.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateJokeDto {
    pub body: String,
}

impl UpdateJokeDto {
    pub fn into_data(self, id: Uuid) -> UpdateJokeData {
        UpdateJokeData {
            id,
            body: self.body.trim().to_string(),
        }
    }
}
