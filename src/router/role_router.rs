use salvo::Router;
use crate::controller::{role_controller};

pub fn init_router()->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/system/role/list").get(role_controller::get_roles_by_page)
    )
        .push(
            Router::with_path("/system/role").post(role_controller::post_add_role).put(role_controller::put_edit_role)
        )
        .push(
            Router::with_path("/system/role/changeStatus").put(role_controller::put_edit_role_status)
        )
        .push(
            Router::with_path("/system/role/<id>").delete(role_controller::del_role_by_id).get(role_controller::get_role_by_id)
        )
        .push(
            Router::with_path("/system/role/authUser/allocatedList").get(role_controller::get_users_by_role_id_page)
        )
        .push(
            Router::with_path("/system/role/authUser/unallocatedList").get(role_controller::get_users_by_not_in_role_id_page)
        )
        .push(
            Router::with_path("/system/role/authUser/cancel").put(role_controller::del_user_role)
        )
        .push(
            Router::with_path("/system/role/authUser/cancelAll").put(role_controller::del_user_role_all)
        )
        .push(
            Router::with_path("/system/role/authUser/selectAll").put(role_controller::put_bind_more_user_and_simple_role)
        )
}