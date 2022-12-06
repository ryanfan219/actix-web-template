use crate::model::{entity::user::User,error::{ServerError}};
use crate::utils;
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserContextDTO {
    pub id: String,
    pub email: Option<String>,
    pub exp: usize,
    pub user_name:String,
}

impl FromRequest for UserContextDTO {
    type Error = ServerError;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready({
            let auth = req.headers().get("Authorization");
            if let Some(val) = auth {
                let token = val
                    .to_str()
                    .unwrap()
                    .split("Bearer ")
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap();
                let result = utils::jwt::validate_token(token);
                match result {
                    Ok(data) => Ok(data.claims),
                    Err(e) => {
                        eprintln!("{}", e);
                        Err(ServerError::create(401, e.to_string()))
                    }
                }
            } else {
                Err(ServerError::create(401, String::from("Authorization Not Found")))
            }
        })
    }
}

impl UserContextDTO {

    pub fn create_with_user(user: User, context: &mut UserContextDTO) -> &mut UserContextDTO {
        context.email = Some(user.email.to_string());
        context.id = user.id.to_string();
        context.user_name = user.username.to_string();
        context
    }
}
