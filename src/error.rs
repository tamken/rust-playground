use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};

/// enum カスタムエラー
#[derive(thiserror::Error, Debug)]
pub enum ApiCustomError {
    /// ルーティング未定義
    #[error("Not Found.")]
    NotFound,

    /// 処理不可
    #[error("Unporcessible Entity.")]
    UnporcessibleEntity(String),

    /// actix内部処理で発生したエラー
    #[error(transparent)]
    ActixWebError(#[from] actix_web::Error),

    /// バリデートエラー
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    // DB(SeaORM)エラー
    #[error(transparent)]
    DbError(#[from] sea_orm::DbErr),

    /// その他
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// 構造体: エラーレスポンスJSON
#[derive(serde::Serialize)]
struct ErrorResponseJson {
    message: String,
}

/// カスタムエラー実装
impl ResponseError for ApiCustomError {
    /// ステータスコード
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiCustomError::NotFound => StatusCode::NOT_FOUND,
            ApiCustomError::UnporcessibleEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiCustomError::ActixWebError(err) => err.as_response_error().status_code(),
            ApiCustomError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiCustomError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiCustomError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// エラーレスポンス
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            ApiCustomError::NotFound => {
                HttpResponse::build(self.status_code()).json(ErrorResponseJson {
                    message: format!("{}", self),
                })
            }
            ApiCustomError::UnporcessibleEntity(message) => HttpResponse::build(self.status_code())
                .json(ErrorResponseJson {
                    message: message.to_string(),
                }),
            ApiCustomError::ActixWebError(err) => {
                let message = match self.status_code() {
                    StatusCode::NOT_FOUND => "Not Found.",
                    _ => "Bad Request.",
                };
                HttpResponse::build(self.status_code()).json(ErrorResponseJson {
                    message: format!("{} [{}]", message, err),
                })
            }
            ApiCustomError::ValidationError(err) => {
                HttpResponse::build(self.status_code()).json(ErrorResponseJson {
                    message: format!("Bad Request. [{}]", err),
                })
            }
            ApiCustomError::DbError(err) => {
                HttpResponse::build(self.status_code()).json(ErrorResponseJson {
                    message: format!("Internal Server Error. [{}]", err),
                })
            }
            ApiCustomError::Other(err) => {
                HttpResponse::build(self.status_code()).json(ErrorResponseJson {
                    message: format!("Internal Server Error. [{}]", err),
                })
            }
        }
    }
}
