use salvo::Router;
use crate::controller::{dict_controller};

pub fn init_router()->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/system/dict/type/list").get(dict_controller::get_dict_list)
    )
        .push(
            Router::with_path("/system/dict/data/list").get(dict_controller::get_dict_data_list)
        )
        .push(
            Router::with_path("/system/dict/data").post(dict_controller::post_add_dict_data).put(dict_controller::put_edit_dict_data)
        )
        .push(
            Router::with_path("/system/dict/data/<id>").delete(dict_controller::del_dict_type_data).get(dict_controller::get_dict_type_data_by_id)
        )
        .push(
            Router::with_path("/system/dict/data/type/<type_id>").get(dict_controller::get_dict_list_by_type)
        )
        .push(
            Router::with_path("/system/dict/type/optionselect").get(dict_controller::get_all_dict_type)
        )
        .push(
            Router::with_path("/system/dict/type/<id>").get(dict_controller::get_dict_by_id).delete(dict_controller::del_dict_type)
        )
        .push(
            Router::with_path("/system/dict/type").post(dict_controller::add_dict_type).put(dict_controller::edit_dict_type)
        )
}