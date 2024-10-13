use salvo::Router;
use crate::controller::notice_controller;

pub fn init_router() ->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/system/notice/list").get(notice_controller::get_notice_page)
    ).push(
        Router::with_path("/system/notice").post(notice_controller::notice_add_notice).put(notice_controller::put_edit_notice)
    ).push(
        Router::with_path("/system/notice/<id>").get(notice_controller::get_notice_by_id).delete(notice_controller::del_notice_by_id)
    )
}