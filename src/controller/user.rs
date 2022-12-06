use crate::model::{
    dto::{auth::UserContextDTO, ResultDTO},
    error::{ServerError, UserError},
    param::user_param::LoginParam,
    vo::user::LoginVO,
};
use crate::service::user_service;
use actix_web::{get, post, web, Result};

#[get("/echo")]
pub async fn echo(
    req_body: String,
    userInfo: Option<UserContextDTO>,
) -> Result<ResultDTO<String>, ServerError> {
    let res = ResultDTO::with_data("test".into());
    Ok(res)
}

#[get("/test-error")]
pub async fn test_error(req_body: String, userInfo: UserContextDTO) -> Result<String, ServerError> {
    let error = ServerError::create_by_message("test".into());
    let user_error = UserError::ValidationError {
        field: ("test".into()),
    };
    Err(user_error.into())
    // Ok(String::from("test"))
}

#[get("/login")]
pub async fn user_login(param: web::Json<LoginParam>) -> Result<ResultDTO<LoginVO>, ServerError> {
    let mut info = param.0;
    user_service::login(&mut info)
}
