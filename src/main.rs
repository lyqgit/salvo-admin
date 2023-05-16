use rbatis::Rbatis;
use once_cell::sync::Lazy;
use rbdc_mysql::driver::MysqlDriver;
use salvo::catcher::Catcher;
use salvo::prelude::CatchPanic;
use salvo::{Server, Listener,Router, Service};
use salvo::conn::TcpListener;
use salvo::serve_static::StaticDir;
use salvo::oapi::swagger_ui::SwaggerUi;
use salvo::oapi::{Info, OpenApi};
use salvo::logging::Logger;
use redis::{Client};


mod controller;
mod service;
mod mapper;
mod model;
mod entity;
mod utils;

use controller::{user_controller,common_controller};

pub static GLOBAL_DB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());

pub static GLOBAL_REDIS:Lazy<Client> = Lazy::new(||{
    let client = redis::Client::open("redis://127.0.0.1/").expect("连接redis失败");
    client.get_connection().unwrap();
    return client;
});

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();
    // 连接数据库
    GLOBAL_DB.link(MysqlDriver {}, "mysql://root:123456@localhost/ry-vue").await.unwrap();


    tracing::info!("Listening on http://127.0.0.1:8080");
    tracing::debug!("Listening on http://127.0.0.1:8080");
    tracing::warn!("Listening on http://127.0.0.1:8080");


    let static_dir = Router::with_path("/static").get(
        StaticDir::new([
            "static/",
        ])
        .listing(true),
    );

    let router = Router::new().hoop(Logger::new())
        .hoop(CatchPanic::new())
        .push(static_dir)
        .push(
            Router::new().push(
                // 需要验证的api
                // Router::with_path("/captchaImage").hoop(common_controller::auth_token).get(user_controller::get_captcha)
                Router::with_path("/captchaImage").get(user_controller::get_captcha)
            ).push(
                Router::with_path("/login").post(user_controller::login)
            )
        );
        // .push(
        //     Router::new().hoop(common_controller::auth_token);
        // );


    let doc = OpenApi::new(Info::new("todos api", "0.0.1")).merge_router(&router);
    let router = router
    .push(doc.into_router("/api-doc/openapi.json"))
    .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"));

    let service = Service::new(router).catcher(Catcher::default().hoop(common_controller::catcher_err));

    Server::new(
        TcpListener::new("0.0.0.0:8081").bind().await
    ).serve(service).await;
}
