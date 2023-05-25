use salvo::oapi::endpoint;
use salvo::oapi::extract::JsonBody;
use salvo::{Depot, Request};
use crate::model::common_model::Page;
use crate::model::role_model::{SysRoleList, SysRoleModifyPayload, SysRolePagePayload};
use crate::service::role_service;
use crate::utils::res::{Res, res_json_ok,match_ok,ResObj};

#[endpoint(
    parameters(
        SysRolePagePayload
    ),
    responses(
        (status = 200,body=ResObj<Page<SysRoleList>>,description ="角色列表")
    ),
)]
pub async fn get_roles_by_page(req:&mut Request)->Res<Page<SysRoleList>>{
    let payload:SysRolePagePayload = req.parse_queries().unwrap();
    match_ok::<Page<SysRoleList>>(role_service::get_role_by_page(
        payload.page_num.map_or(1,|v|v),
        payload.page_size.map_or(10,|v|v),
        payload.role_name,
        payload.role_key,
        payload.begin_time,
        payload.end_time,
    ).await)
}

#[endpoint(
    responses(
        (status = 200,body=ResObj<bool>,description ="添加角色")
    ),
)]
pub async fn post_add_role(payload:JsonBody<SysRoleModifyPayload>, depot:&mut Depot) -> Res<bool> {
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    let payload = payload.into_inner();
    match_ok(role_service::add_role_and_bind_menu(user_id,payload.dept_check_strictly,payload.menu_check_strictly,payload.menu_ids,payload.role_key,payload.role_name,payload.status,payload.role_sort,payload.remark).await)
}
