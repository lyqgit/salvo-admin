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

use controller::{user_controller,common_controller,dict_controller};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub static GLOBAL_DB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());

pub static GLOBAL_REDIS:Lazy<Client> = Lazy::new(||{
    let client = redis::Client::open("redis://127.0.0.1/").expect("连接redis失败");
    client.get_connection().unwrap();
    return client;
});

#[tokio::main]
async fn main() {

    // let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        // .with_max_level(Level::INFO)
        // completes the builder.
        // .finish();

    
    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    
    // 连接数据库
    GLOBAL_DB.link(MysqlDriver {}, "mysql://root:123456@localhost/ry-vue").await.unwrap();

    // tracing::warn!("Listening on http://127.0.0.1:8080");


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
                
                // Router::with_path("/captchaImage").hoop(common_controller::auth_token).get(user_controller::get_captcha)
                Router::with_path("/captchaImage").get(user_controller::get_captcha)
            ).push(
                Router::with_path("/login").post(user_controller::login)
            ).push(
                Router::with_path("/logout").post(user_controller::log_out)
            )
        )
        .push(
            // 需要验证的api
            Router::new().hoop(common_controller::auth_token)
            .push(
                Router::with_path("/getInfo").get(user_controller::get_info)
            )
            .push(
                Router::with_path("/getRouters").get(user_controller::get_routers)
            )

            .push(
                Router::with_path("/system/dict/type/list").get(dict_controller::get_dict_list)
            )
            .push(
                Router::with_path("/system/dict/data/list").get(dict_controller::get_dict_data_list)
            )
            .push(
                Router::with_path("/system/dict/data").post(dict_controller::post_add_dict_data)
            )
            .push(
                Router::with_path("/system/dict/data/<id>").delete(dict_controller::del_dict_type_data)
            )
            .push(
                Router::with_path("/system/dict/data/type/<type>").get(dict_controller::get_dict_list_by_type)
            )
            .push(
                Router::with_path("/system/dict/type/optionselect").get(dict_controller::get_all_dict_type)
            )
            .push(
                Router::with_path("/system/dict/type/<id>").get(dict_controller::get_dict_by_id).delete(dict_controller::del_dict_type)
            )
            .push(
                Router::with_path("/system/dict/type").post(dict_controller::add_dict_type).put(dict_controller::edit_dict_type)
            )
        );


    let doc = OpenApi::new(Info::new("todos api", "0.0.1")).merge_router(&router);
    let router = router
    .push(doc.into_router("/api-doc/openapi.json"))
    .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"));

    let service = Service::new(router).catcher(Catcher::default().hoop(common_controller::catcher_err));

    Server::new(
        TcpListener::new("0.0.0.0:8080").bind().await
    ).serve(service).await;
}
