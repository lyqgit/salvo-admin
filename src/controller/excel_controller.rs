use salvo::{Depot, handler, Request, Response};
use salvo::http::StatusError;
use salvo::oapi::endpoint;
use salvo::oapi::extract::{JsonBody, PathParam};
use crate::model::excel_model::{ExcelModifyPayload,ExcelList,ExcelDetail};
use crate::service::excel_service;
use crate::utils::res::{ Res,ResObj,match_ok,match_no_res_ok};
use salvo::websocket::WebSocketUpgrade;
use crate::websocket::excel_websocket;

/// 添加文档
#[endpoint(
    tags("文档"),
    responses(
        (status_code = 200,body=ResObj<String>,description ="添加文档")
    ),
)]
pub async fn post_add_excel(payload:JsonBody<ExcelModifyPayload>,depot:&mut Depot)->Res<String>{
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    match_ok(
    excel_service::add_excel(
            payload.excel_name.clone(),
            payload.excel_data.clone(),
            user_id
        ).await
    )
}

/// 添加文档
#[endpoint(
    tags("文档"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="修改文档")
    ),
)]
pub async fn post_edit_excel(payload:JsonBody<ExcelModifyPayload>)->Res<()>{
    match_no_res_ok(
    excel_service::edit_excel(
            payload.excel_id.clone(),
            payload.excel_data.clone()
        ).await
    )
}

/// 添加文档
#[endpoint(
    tags("文档"),
    responses(
        (status_code = 200,body=ResObj<Vec<ExcelList>>,description ="查询个人的文档列表")
    ),
)]
pub async fn get_excel_list_by_user_id(depot:&mut Depot)->Res<Vec<ExcelList>>{
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    match_ok(excel_service::excel_list_by_user_id(user_id).await)
}

/// 添加文档
#[endpoint(
    tags("文档"),
    responses(
        (status_code = 200,body=ResObj<Vec<ExcelList>>,description ="文档列表")
    ),
)]
pub async fn get_excel_list()->Res<Vec<ExcelList>>{
    match_ok(excel_service::excel_list().await)
}

/// 添加文档
#[endpoint(
    tags("文档"),
    responses(
        (status_code = 200,body=ResObj<Option<ExcelDetail>>,description ="文档详情")
    ),
)]
pub async fn get_excel_detail_by_excel_id(excel_id:PathParam<Option<String>>)->Res<Option<ExcelDetail>>{
    let id = excel_id.into_inner().map_or(String::from(""), |v|v);
    match_ok(excel_service::excel_detail(id).await)
}

/// 协同excel
#[handler]
pub async fn excel_connected(req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
    WebSocketUpgrade::new().upgrade(req, res, excel_websocket::excel_handle_socket).await
}