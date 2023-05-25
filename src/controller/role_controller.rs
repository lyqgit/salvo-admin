use salvo::oapi::endpoint;
use salvo::Request;
use crate::model::common_model::Page;
use crate::model::role_model::{SysRoleList, SysRolePagePayload};
use crate::service::role_service;
use crate::utils::res::{Res, res_json_ok,promise_ok,ResObj};

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
    promise_ok::<Page<SysRoleList>>(role_service::get_role_by_page(
        payload.page_num.map_or(1,|v|v),
        payload.page_size.map_or(10,|v|v),
        payload.role_name,
        payload.role_key,
        payload.begin_time,
        payload.end_time,
    ).await,Box::new(|v:Page<SysRoleList>|res_json_ok(Some(v))))
}
