use salvo::Router;
use crate::controller::excel_controller;

pub fn init_router()->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/tool/excel/add").post(excel_controller::post_add_excel)
    ).push(
        Router::with_path("/tool/excel/edit").post(excel_controller::post_edit_excel)
    ).push(
        Router::with_path("/tool/excel/user/list").get(excel_controller::get_excel_list_by_user_id)
    ).push(
        Router::with_path("/tool/excel/list").get(excel_controller::get_excel_list)
    ).push(
        Router::with_path("/tool/excel/<excel_id>").get(excel_controller::get_excel_detail_by_excel_id)
    )
}