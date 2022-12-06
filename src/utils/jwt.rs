use crate::model::dto::auth::UserContextDTO;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey,errors::{Error} ,Header, TokenData, Validation,};

const JWT_SECRET: &[u8] = b"Jwt_Secret";

/// 创建token
pub fn create_jwt(context: &mut UserContextDTO) -> String {
    let mut header = Header::new(Algorithm::HS512);
    let expiration = Utc::now()
    .checked_add_signed(chrono::Duration::seconds(3600))
    .expect("valid timestamp")
    .timestamp(); 
    context.exp = expiration as usize;
    let token =
        jsonwebtoken::encode(&header, context, &EncodingKey::from_secret(JWT_SECRET)).unwrap();
    format!("Bearer {}", token)
}

/// 验证token
pub fn validate_token(token: &str) -> Result<TokenData<UserContextDTO>,Error> {
    let validation = Validation::new(Algorithm::HS512);
    let key = DecodingKey::from_secret(JWT_SECRET);
    jsonwebtoken::decode::<UserContextDTO>(token, &key, &validation)
}
