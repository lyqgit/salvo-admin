use salvo::oapi::endpoint;
use salvo::oapi::extract::{JsonBody, PathParam};
use salvo::{Depot, Request};
use crate::model::common_model::Page;
use crate::model::role_model::{SysRoleList, SysRoleModifyPayload, SysRolePagePayload, SysRoleStatusPayload, SysUserRolePagePayload};
use crate::model::user_model::SysUserList;
use crate::service::role_service;
use crate::utils::res::{Res, match_ok, ResObj, match_no_res_ok};

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
        payload.status,
        payload.begin_time,
        payload.end_time,
    ).await)
}

#[endpoint(
    responses(
        (status = 200,body=ResObj<()>,description ="添加角色")
    ),
)]
pub async fn post_add_role(payload:JsonBody<SysRoleModifyPayload>, depot:&mut Depot) -> Res<()> {
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    let payload = payload.into_inner();
    match_no_res_ok(role_service::add_role_and_bind_menu(user_id,payload.dept_check_strictly,payload.menu_check_strictly,payload.menu_ids,payload.role_key,payload.role_name,payload.status,payload.role_sort,payload.remark).await)
}

#[endpoint(
    responses(
        (status = 200,body=ResObj<()>,description ="更改角色状态")
    ),
)]
pub async fn put_edit_role_status(payload:JsonBody<SysRoleStatusPayload>)->Res<()>{
    match_no_res_ok(role_service::update_role_status_by_id(payload.role_id,payload.status.clone()).await)
}

#[endpoint(
    responses(
        (status = 200,body=ResObj<()>,description ="删除角色")
    ),
)]
pub async fn del_role_by_id(id:PathParam<String>)->Res<()>{
    match_no_res_ok(role_service::del_role_by_id(id.into_inner()).await)
}

#[endpoint(
    responses(
        (status = 200,body=ResObj<Option<SysRoleList>>,description ="获取角色详情")
    ),
)]
pub async fn get_role_by_id(id:PathParam<String>)->Res<Option<SysRoleList>>{
    match_ok(role_service::get_role_by_id(id.into_inner()).await)
}

#[endpoint(
    responses(
        (status = 200,body=ResObj<()>,description ="修改角色")
    ),
)]
pub async fn put_edit_role(payload:JsonBody<SysRoleModifyPayload>, depot:&mut Depot) -> Res<()> {
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    let payload = payload.into_inner();
    match_no_res_ok(role_service::edit_role_and_bind_menu(
        user_id,
        payload.dept_check_strictly,
        payload.menu_check_strictly,
        payload.menu_ids,
        payload.role_key,
        payload.role_name,
        payload.status,
        payload.data_scope.unwrap(),
        payload.del_flag.unwrap(),
        payload.create_by.unwrap(),
        payload.create_time.unwrap(),
        payload.role_sort,
        payload.remark,
        payload.role_id.unwrap()
    ).await)
}


#[endpoint(
    parameters(
        SysUserRolePagePayload
    ),
    responses(
        (status = 200,body=ResObj<Page<SysUserList>>,description ="根据角色id获取用户列表")
    ),
)]
pub async fn get_users_by_role_id_page(req:&mut Request)->Res<Page<SysUserList>>{
    let payload:SysUserRolePagePayload = req.parse_queries().unwrap();
    match_ok(role_service::select_users_by_role_id(payload.user_name,payload.phone_number,payload.role_id,payload.page_num.map_or(1,|v|v),payload.page_size.map_or(10,|v|v),).await)
}