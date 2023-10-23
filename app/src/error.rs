pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // ! GENERAL
    #[error("Unexpected Error: {0}")]
    Unexpected(#[from] anyhow::Error),
    #[error("Internal Error: {0}")]
    Internal(String),
    #[error("Not Implemented: {0}")]
    NotImplemented(String),
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Duplicate: {0}")]
    Duplicate(String),

    // ! DB
    #[error("Db Error: {0}")]
    DbErr(#[source] sea_orm::DbErr),

    // ! ASSETS
    #[error("Asset Error: {0}")]
    AssetError(String),
}
