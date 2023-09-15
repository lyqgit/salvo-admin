use salvo::oapi::endpoint;
use salvo::oapi::extract::{JsonBody, PathParam};
use salvo::{Depot, Request};
use crate::model::dept_model::{DeptList, DeptListPayload, DeptModifyPayload, DeptTree};
use crate::service::dept_service;
use crate::utils::res::{match_no_res_ok, match_ok, Res, ResObj};

/// 部门列表
#[endpoint(
    tags("部门"),
    parameters(
        DeptListPayload
    ),
    responses(
        (status_code = 200,body=ResObj<Vec<DeptList>>,description ="部门列表")

    ),
)]
pub async fn get_dept_list(req:&mut Request)->Res<Vec<DeptList>>{
    let payload = req.parse_queries::<DeptListPayload>().map_or(DeptListPayload{ dept_name: None, status: None },|v|v);
    match_ok(dept_service::get_dept_list(payload.dept_name,payload.status).await)
}

/// 去除传入id部门的部门列表
#[endpoint(
    tags("部门"),
    responses(
        (status_code = 200,body=ResObj<Vec<DeptList>>,description ="去除传入id部门的部门列表")
    ),
)]
pub async fn get_dept_list_exclude_id(id:PathParam<i64>)->Res<Vec<DeptList>>{
    match_ok(dept_service::get_dept_list_exclude_id(id.into_inner()).await)
}

/// 添加部门
#[endpoint(
    tags("部门"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="添加部门")
    ),
)]
pub async fn post_add_dept(payload:JsonBody<DeptModifyPayload>,depot:&mut Depot)->Res<()>{
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    match_no_res_ok(dept_service::add_dept(
        user_id,
        payload.dept_name.clone(),
        payload.order_num,payload.parent_id,
        payload.leader.clone(),
        payload.email.clone(),
        payload.phone.clone(),
        payload.status.clone()
    ).await)
}

/// 修改部门
#[endpoint(
    tags("部门"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="修改部门")
    ),
)]
pub async fn put_edit_dept(payload:JsonBody<DeptModifyPayload>,depot:&mut Depot)->Res<()>{
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    match_no_res_ok(dept_service::edit_dept(
        user_id,
        payload.dept_id.clone(),
        payload.dept_name.clone(),
        payload.order_num,payload.parent_id,
        payload.leader.clone(),
        payload.email.clone(),
        payload.phone.clone(),
        payload.status.clone()
    ).await)
}

/// 删除部门
#[endpoint(
    tags("部门"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="删除部门")
    ),
)]
pub async fn del_dept_by_id(id:PathParam<String>)->Res<()>{
    match_no_res_ok(dept_service::del_dept_by_id(
        id.into_inner()
    ).await)
}

/// 部门详情
#[endpoint(
    tags("部门"),
    responses(
        (status_code = 200,body=ResObj<Option<DeptList>>,description ="部门详情")
    ),
)]
pub async fn get_dept_by_id(id:PathParam<i64>)->Res<Option<DeptList>>{
    match_ok(dept_service::get_dept_by_id(id.into_inner()).await)
}

/// 部门树
#[endpoint(
    tags("部门"),
    responses(
        (status_code = 200,body=ResObj<Vec<DeptTree>>,description ="部门树")
    ),
)]
pub async fn get_dept_tree()->Res<Vec<DeptTree>>{
    match_ok(dept_service::get_dept_tree().await)
}