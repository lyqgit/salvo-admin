use salvo::oapi::endpoint;
use salvo::oapi::extract::{JsonBody, PathParam};
use salvo::{Depot, Request};
use crate::model::common_model::Page;
use crate::model::role_model::{SysRoleCancelUserPayload, SysRoleList, SysRoleModifyPayload, SysRolePagePayload, SysRoleStatusPayload, SysUserRoleCancelPayload, SysUserRolePagePayload};
use crate::model::user_model::SysUserList;
use crate::service::role_service;
use crate::utils::res::{Res, match_ok, ResObj, match_no_res_ok};

/// 角色列表
#[endpoint(
    tags("角色"),
    parameters(
        SysRolePagePayload
    ),
    responses(
        (status_code = 200,body=ResObj<Page<SysRoleList>>,description ="角色列表")
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

/// 添加角色
#[endpoint(
    tags("角色"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="添加角色")
    ),
)]
pub async fn post_add_role(payload:JsonBody<SysRoleModifyPayload>, depot:&mut Depot) -> Res<()> {
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    let payload = payload.into_inner();
    match_no_res_ok(role_service::add_role_and_bind_menu(user_id,payload.dept_check_strictly,payload.menu_check_strictly,payload.menu_ids,payload.role_key,payload.role_name,payload.status,payload.role_sort,payload.remark).await)
}

/// 更改角色状态
#[endpoint(
    tags("角色"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="更改角色状态")
    ),
)]
pub async fn put_edit_role_status(payload:JsonBody<SysRoleStatusPayload>)->Res<()>{
    match_no_res_ok(role_service::update_role_status_by_id(payload.role_id,payload.status.clone()).await)
}

/// 删除角色
#[endpoint(
    tags("角色"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="删除角色")
    ),
)]
pub async fn del_role_by_id(id:PathParam<String>)->Res<()>{
    match_no_res_ok(role_service::del_role_by_id(id.into_inner()).await)
}

/// 获取角色详情
#[endpoint(
    tags("角色"),
    responses(
        (status_code = 200,body=ResObj<Option<SysRoleList>>,description ="获取角色详情")
    ),
)]
pub async fn get_role_by_id(id:PathParam<String>)->Res<Option<SysRoleList>>{
    match_ok(role_service::get_role_by_id(id.into_inner()).await)
}

/// 修改角色
#[endpoint(
    tags("角色"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="修改角色")
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

/// 根据角色id获取用户列表
#[endpoint(
    tags("角色"),
    parameters(
        SysUserRolePagePayload
    ),
    responses(
        (status_code = 200,body=ResObj<Page<SysUserList>>,description ="根据角色id获取用户列表")
    ),
)]
pub async fn get_users_by_role_id_page(req:&mut Request)->Res<Page<SysUserList>>{
    let payload:SysUserRolePagePayload = req.parse_queries().unwrap();
    match_ok(role_service::select_users_by_role_id(payload.user_name,payload.phone_number,payload.role_id,payload.page_num.map_or(1,|v|v),payload.page_size.map_or(10,|v|v),).await)
}

/// 根据角色id获取非此角色的用户列表
#[endpoint(
    tags("角色"),
    parameters(
        SysUserRolePagePayload
    ),
    responses(
        (status_code = 200,body=ResObj<Page<SysUserList>>,description ="根据角色id获取非此角色的用户列表")
    ),
)]
pub async fn get_users_by_not_in_role_id_page(req:&mut Request)->Res<Page<SysUserList>>{
    let payload:SysUserRolePagePayload = req.parse_queries().unwrap();
    match_ok(role_service::select_users_not_in_role_id(payload.user_name,payload.phone_number,payload.role_id,payload.page_num.map_or(1,|v|v),payload.page_size.map_or(10,|v|v),).await)
}

/// 删除角色和用户绑定关系
#[endpoint(
    tags("角色"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="删除角色和用户绑定关系")
    ),
)]
pub async fn del_user_role(payload:JsonBody<SysRoleCancelUserPayload>) -> Res<()> {
    match_no_res_ok(role_service::del_user_role_bind(
        payload.user_id,
        payload.role_id.clone()
    ).await)
}

/// 删除多个角色和用户绑定关系
#[endpoint(
    tags("角色"),
    parameters(
        SysUserRoleCancelPayload
    ),
    responses(
        (status_code = 200,body=ResObj<()>,description ="删除多个角色和用户绑定关系")
    ),
)]
pub async fn del_user_role_all(req:&mut Request) -> Res<()> {
    let payload:SysUserRoleCancelPayload = req.parse_queries().unwrap();
    match_no_res_ok(role_service::del_user_role_bind_more(
        payload.user_ids.clone(),
        payload.role_id
    ).await)
}

/// 绑定多个用户和一个角色
#[endpoint(
    tags("角色"),
    parameters(
        SysUserRoleCancelPayload
    ),
    responses(
        (status_code = 200,body=ResObj<()>,description ="绑定多个用户和一个角色")
    ),
)]
pub async fn put_bind_more_user_and_simple_role(req:&mut Request) -> Res<()> {
    let payload:SysUserRoleCancelPayload = req.parse_queries().unwrap();
    match_no_res_ok(role_service::bind_more_user_and_simple_role(
        payload.user_ids.clone(),
        payload.role_id
    ).await)
}