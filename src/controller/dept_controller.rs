use salvo::oapi::endpoint;
use salvo::oapi::extract::{JsonBody, PathParam};
use salvo::{Depot, Request};
use crate::model::dept_model::{DeptList, DeptListPayload, DeptModifyPayload};
use crate::service::dept_service;
use crate::utils::res::{match_no_res_ok, match_ok, Res, ResObj};

#[endpoint(
    parameters(
        DeptListPayload
    ),
    responses(
        (status = 200,body=ResObj<Vec<DeptList>>,description ="部门列表")
    ),
)]
pub async fn get_dept_list(req:&mut Request)->Res<Vec<DeptList>>{
    let payload = req.parse_queries::<DeptListPayload>().map_or(DeptListPayload{ dept_name: None, status: None },|v|v);
    match_ok(dept_service::get_dept_list(payload.dept_name,payload.status).await)
}

#[endpoint(
    responses(
        (status = 200,body=ResObj<Vec<DeptList>>,description ="去除传入id的部门列表")
    ),
)]
pub async fn get_dept_list_exclude_id(id:PathParam<i64>)->Res<Vec<DeptList>>{
    match_ok(dept_service::get_dept_list_exclude_id(id.into_inner()).await)
}

#[endpoint(
    responses(
        (status = 200,body=ResObj<()>,description ="添加部门")
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

#[endpoint(
    responses(
        (status = 200,body=ResObj<()>,description ="修改部门")
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

#[endpoint(
    responses(
        (status = 200,body=ResObj<Option<DeptList>>,description ="修改部门")
    ),
)]
pub async fn get_dept_by_id(id:PathParam<i64>)->Res<Option<DeptList>>{
    match_ok(dept_service::get_dept_by_id(id.into_inner()).await)
}