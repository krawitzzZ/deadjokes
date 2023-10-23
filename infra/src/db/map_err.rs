use std::borrow::Cow;

use app::AppError;

const DUPLICATE_ERROR_CODE: Cow<'_, str> = Cow::Borrowed("23505");

pub fn into_app_db_err(error: sea_orm::DbErr) -> AppError {
    if let Some(reason) = get_reason_for_code(&error, DUPLICATE_ERROR_CODE) {
        return AppError::Duplicate(reason);
    }

    AppError::DbErr(error)
}

fn get_reason_for_code(error: &sea_orm::DbErr, code: Cow<'_, str>) -> Option<String> {
    match error {
        sea_orm::DbErr::Exec(runtime_err) => match runtime_err {
            sea_orm::RuntimeErr::SqlxError(err) => get_sqlx_reason_for_code(err, code),
            sea_orm::RuntimeErr::Internal(_) => None,
        },
        sea_orm::DbErr::Query(err) => match err {
            sea_orm::RuntimeErr::SqlxError(err) => get_sqlx_reason_for_code(err, code),
            sea_orm::RuntimeErr::Internal(_) => None,
        },
        sea_orm::DbErr::Conn(err) => match err {
            sea_orm::RuntimeErr::SqlxError(err) => get_sqlx_reason_for_code(err, code),
            sea_orm::RuntimeErr::Internal(_) => None,
        },
        sea_orm::DbErr::ConnectionAcquire(_) => None,
        sea_orm::DbErr::TryIntoErr { .. } => None,
        sea_orm::DbErr::ConvertFromU64(_) => None,
        sea_orm::DbErr::UnpackInsertId => None,
        sea_orm::DbErr::UpdateGetPrimaryKey => None,
        sea_orm::DbErr::RecordNotFound(_) => None,
        sea_orm::DbErr::AttrNotSet(_) => None,
        sea_orm::DbErr::Custom(_) => None,
        sea_orm::DbErr::Type(_) => None,
        sea_orm::DbErr::Json(_) => None,
        sea_orm::DbErr::Migration(_) => None,
        sea_orm::DbErr::RecordNotInserted => None,
        sea_orm::DbErr::RecordNotUpdated => None,
    }
}

fn get_sqlx_reason_for_code(err: &sqlx::Error, code: Cow<'_, str>) -> Option<String> {
    match err {
        sqlx::Error::Database(e) if e.code() == Some(code) => Some(e.message().into()),
        _ => None,
    }
}
