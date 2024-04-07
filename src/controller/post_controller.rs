use salvo::{oapi::endpoint,Writer};
use salvo::oapi::extract::{JsonBody, PathParam};
use salvo::{Depot, Request};
use crate::model::common_model::Page;
use crate::model::post_model::{SysPostList, SysPostListPayload, SysPostModifyPayload};
use crate::service::post_service;
use crate::utils::res::{match_no_res_ok, match_ok, Res, ResObj};

/// 岗位列表
#[endpoint(
    tags("岗位"),
    parameters(
        SysPostListPayload
    ),
    responses(
        (status_code = 200,body=ResObj<Page<SysPostList>>,description ="岗位列表")
    ),
)]
pub async fn get_post_page(req:&mut Request)->Res<Page<SysPostList>>{
    let payload:SysPostListPayload = req.parse_queries().unwrap();
    match_ok(post_service::get_post_page(payload.page_num,payload.page_size,payload.post_code,payload.post_name,payload.status).await)
}

/// 添加岗位
#[endpoint(
    tags("岗位"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="添加岗位")
    ),
)]
pub async fn post_add_post(payload:JsonBody<SysPostModifyPayload>,depot:&mut Depot)->Res<()>{
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    match_no_res_ok(post_service::post_add_post(user_id, payload.post_code.clone(), payload.post_name.clone(), payload.post_sort, payload.status.clone(), payload.remark.clone()).await)
}

/// 修改岗位
#[endpoint(
    tags("岗位"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="修改岗位")
    ),
)]
pub async fn put_edit_post(payload:JsonBody<SysPostModifyPayload>,depot:&mut Depot)->Res<()>{
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    match_no_res_ok(post_service::post_edit_post(user_id,payload.post_id, payload.post_code.clone(), payload.post_name.clone(), payload.post_sort, payload.status.clone(), payload.remark.clone()).await)
}

/// 岗位详情
#[endpoint(
    tags("岗位"),
    responses(
        (status_code = 200,body=ResObj<Option<SysPostList>>,description ="岗位详情")
    ),
)]
pub async fn get_post_by_id(id:PathParam<i64>)->Res<Option<SysPostList>>{
    match_ok(post_service::get_post_by_id(id.into_inner()).await)
}

/// 删除岗位
#[endpoint(
    tags("岗位"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="删除岗位")
    ),
)]
pub async fn del_post_by_id(id:PathParam<String>)->Res<()>{
    match_no_res_ok(post_service::del_post_by_id(id.into_inner()).await)
}
