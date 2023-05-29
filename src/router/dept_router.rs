use salvo::Router;
use crate::controller::{dept_controller};

pub fn init_router()->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/system/dept/list").get(dept_controller::get_dept_list)
    ).push(
        Router::with_path("/system/dept").post(dept_controller::post_add_dept).put(dept_controller::put_edit_dept)
    ).push(
        Router::with_path("/system/dept/<id>").get(dept_controller::get_dept_by_id).delete(dept_controller::del_dept_by_id)
    ).push(
        Router::with_path("/system/dept/list/exclude/<id:num>").get(dept_controller::get_dept_list_exclude_id)
    )
}