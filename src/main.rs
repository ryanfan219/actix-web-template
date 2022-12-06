use actix_web::{
    dev,
    http::StatusCode,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, App, HttpResponse, HttpServer, Result,
};
use log::{info,debug,error};
use log4rs;
mod config;
mod controller;
mod db;
mod handler;
mod model;
mod utils;
mod common;
mod macros;
mod service;

use controller::routes;

#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;

fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init(); //正式环境可以注释此行 ***
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    // info!("启动成功");
    // error!("测试异常日志");
    let port = config::server_config::SETTING.app.port;
    let host_port = &format!("{}:{}", "127.0.0.1", port); //地址/端口
    HttpServer::new(|| {
        App::new()
            .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header))
            .configure(routes)
            .default_service(web::route().to(HttpResponse::NotFound))
    })
    .bind(host_port)?
    .workers(2)
    .run()
    .await
}
