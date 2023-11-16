use rbatis::Rbatis;
use once_cell::sync::Lazy;
use salvo::{Server, Listener};
use salvo::conn::TcpListener;
use redis::{Client};


mod controller;
mod service;
mod mapper;
mod model;
mod entity;
mod utils;
mod router;

pub static GLOBAL_DB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());

pub static GLOBAL_REDIS:Lazy<Client> = Lazy::new(||{
    let client = Client::open("redis://127.0.0.1/").expect("连接redis失败");
    return client;
});

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt().init();

    // 连接数据库
    utils::mysql::init_db().await;

    // 连接redis
    GLOBAL_REDIS.get_connection().unwrap();

    let service = router::init_service();

    Server::new(
        TcpListener::new("0.0.0.0:8090").bind().await
    ).serve(service).await;
}
