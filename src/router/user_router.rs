use salvo::Router;
use crate::controller::{chat_controller, dept_controller, user_controller, excel_controller};

pub fn init_router()->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/getInfo").get(user_controller::get_info)
    ).push(
        Router::with_path("/getRouters").get(user_controller::get_routers)
    )
    .push(
        Router::with_path("/system/user/deptTree").get(dept_controller::get_dept_tree)
    )
    .push(
        Router::with_path("/system/user/list").get(user_controller::get_user_page)
    )
    .push(
        Router::with_path("/system/user/<id>").get(user_controller::get_user_detail).delete(user_controller::del_user)
    )
    .push(
        Router::with_path("/system/user/").get(user_controller::get_dept_and_role)
    )
    .push(
        Router::with_path("/system/user/changeStatus").put(user_controller::put_change_status_by_id)
    )
    .push(
        Router::with_path("/system/user").post(user_controller::post_add_user).put(user_controller::put_edit_user)
    )
    .push(
        Router::with_path("/system/user/resetPwd").put(user_controller::update_user_pwd)
    )
    .push(
        Router::with_path("/system/user/authRole/<id>").get(user_controller::get_user_auth_role_by_id)
    )
    .push(
        Router::with_path("/system/user/authRole").put(user_controller::add_user_and_role)
    )
    .push(
        // 退出登录
        Router::with_path("/logout").post(user_controller::log_out)
    )
}

pub fn init_router_no_token()->Router{
    let router = Router::new();
    router.push(
                // 验证码
        Router::with_path("/captchaImage").get(user_controller::get_captcha)
    )
    .push(
            // 登录
    Router::with_path("/login").post(user_controller::login)
    )
    .push(
        // chat群聊
        Router::with_path("/chat").goal(chat_controller::user_connected)
    ).push(
        Router::with_path("/tool/excel/connected").goal(excel_controller::excel_connected)
    )

}