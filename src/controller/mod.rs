use std::f32::consts::E;

use actix_web::{web, App, HttpResponse, HttpServer};

pub mod user;

use user::{test_error, user_login, echo};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(echo).service(test_error).service(user_login);
}
