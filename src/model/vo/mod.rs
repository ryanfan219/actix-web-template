use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Result<T>
where
    T: Serialize,
{
    body: T,
    code: u16,
    message: String,
}

// Responder
impl<T> Responder for Result<T>
where
    T: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

impl<T> Result<T>
where
    T: Serialize,
{
    pub fn with_data(data: T) -> Self {
        Self {
            body: data,
            code: 200,
            message: "success".to_string(),
        }
    }
}


pub mod user;