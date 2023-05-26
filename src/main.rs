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
    let client = redis::Client::open("redis://127.0.0.1/").expect("连接redis失败");
    client.get_connection().unwrap();
    return client;
});

#[tokio::main]
async fn main() {

    // 连接数据库
    utils::mysql::init_db().await;

    let service = router::init_service();

    Server::new(
        TcpListener::new("0.0.0.0:8080").bind().await
    ).serve(service).await;
}
