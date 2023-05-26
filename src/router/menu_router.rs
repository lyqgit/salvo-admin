use salvo::Router;
use crate::controller::{menu_controller};

pub fn init_router()->Router{
    let router = Router::new();
    router.push(
            Router::with_path("/system/menu/list").get(menu_controller::get_menu_list)
        )
        .push(
            Router::with_path("/system/menu").post(menu_controller::add_menu).put(menu_controller::put_edit_menu)
        )
        .push(
            Router::with_path("/system/menu/<id:num>").delete(menu_controller::del_menu_by_id).get(menu_controller::get_menu_by_id)
        )
        .push(
        Router::with_path("/system/menu/treeselect").get(menu_controller::get_menu_tree)
        )
        .push(
        Router::with_path("/system/menu/roleMenuTreeselect/<id:num>").get(menu_controller::get_role_menu_tree_by_user_id)
        )
}