use salvo::Router;
use crate::controller::monitor_controller;

pub fn init_router()->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/monitor/server").get(monitor_controller::get_server_info)
    )
}