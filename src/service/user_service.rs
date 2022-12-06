use crate::model::{
    vo::{user::{
        LoginVO
    }},
    entity::{user::{User}},
    param::{user_param::{
        LoginParam
    }}, dto::ResultDTO, error::ServerError
};
use actix_web::{Result};

pub fn login(param:& mut LoginParam)->Result<ResultDTO<LoginVO>, ServerError>{
    Ok(ResultDTO::with_data(LoginVO {  }))
}