use salvo::Router;
use crate::controller::{dept_controller};

pub fn init_router()->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/system/dept/list").get(dept_controller::get_dept_list)
    )
}