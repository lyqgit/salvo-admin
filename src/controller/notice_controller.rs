use salvo::{oapi::endpoint,Writer};
use salvo::oapi::extract::{JsonBody, PathParam};
use salvo::{Depot, Request};
use crate::model::common_model::Page;
use crate::model::notice_model::{SysNoticeList, SysNoticeListPayload, SysNoticeModifyPayload};
use crate::service::notice_service;
use crate::utils::res::{match_no_res_ok, match_ok, Res, ResObj};

/// 通知列表
#[endpoint(
    tags("通知"),
    parameters(
        SysNoticeListPayload
    ),
    responses(
        (status_code = 200,body=ResObj<Page<SysNoticeList>>,description ="通知列表")
    ),
)]
pub async fn get_notice_page(req:&mut Request)->Res<Page<SysNoticeList>>{
    let payload:SysNoticeListPayload = req.parse_queries().unwrap();
    match_ok(notice_service::get_notice_page(payload.page_num,payload.page_size,payload.notice_title,payload.notice_type,payload.status).await)
}

/// 添加通知
#[endpoint(
    tags("通知"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="添加通知")
    ),
)]
pub async fn notice_add_notice(payload:JsonBody<SysNoticeModifyPayload>,depot:&mut Depot)->Res<()>{
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    match_no_res_ok(notice_service::notice_add_notice(user_id, payload.notice_title.clone(), payload.notice_type.clone(), payload.notice_content.clone(), payload.status.clone(), payload.remark.clone()).await)
}

/// 修改通知
#[endpoint(
    tags("通知"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="修改通知")
    ),
)]
pub async fn put_edit_notice(payload:JsonBody<SysNoticeModifyPayload>,depot:&mut Depot)->Res<()>{
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    match_no_res_ok(notice_service::notice_edit_notice(user_id,payload.notice_id, payload.notice_title.clone(), payload.notice_type.clone(), payload.notice_content.clone(), payload.status.clone(), payload.remark.clone()).await)
}

/// 通知详情
#[endpoint(
    tags("通知"),
    responses(
        (status_code = 200,body=ResObj<Option<SysNoticeList>>,description ="通知详情")
    ),
)]
pub async fn get_notice_by_id(id:PathParam<i64>)->Res<Option<SysNoticeList>>{
    match_ok(notice_service::get_notice_by_id(id.into_inner()).await)
}

/// 删除通知
#[endpoint(
    tags("通知"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="删除通知")
    ),
)]
pub async fn del_notice_by_id(id:PathParam<String>)->Res<()>{
    match_no_res_ok(notice_service::del_notice_by_id(id.into_inner()).await)
}
