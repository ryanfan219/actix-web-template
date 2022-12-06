use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum UserError {
    #[error("Not found for table: {field}")]
    ValidationError { field: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerError {
    code: u16,
    message: String,
}

impl From<UserError> for ServerError {
    fn from(value: UserError) -> Self {
        Self { code: 500, message: value.to_string() }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code:{}, message:{}", self.code, self.message)
    }
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&self).unwrap())
    }
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl ServerError {
    pub fn create(code: u16, message: String) -> Self {
        Self {
            code: code,
            message: message,
        }
    }

    pub fn create_by_code(code: u16) -> Self {
        Self {
            code: code,
            message: "server error".into(),
        }
    }

    pub fn create_by_message(message: String) -> Self {
        Self {
            code: 500,
            message: message,
        }
    }
}
