use salvo::Router;
use crate::controller::post_controller;

pub fn init_router() ->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/system/post/list").get(post_controller::get_post_page)
    ).push(
        Router::with_path("/system/post").post(post_controller::post_add_post).put(post_controller::put_edit_post)
    ).push(
        Router::with_path("/system/post/<id>").get(post_controller::get_post_by_id).delete(post_controller::del_post_by_id)
    )
}