use salvo::oapi::endpoint;
use salvo::oapi::extract::QueryParam;
use salvo::Request;
use crate::model::dept_model::{DeptList, DeptListPayload};
use crate::service::dept_service;
use crate::utils::res::{match_ok, Res,ResObj};

#[endpoint(
    parameters(
        DeptListPayload
    ),
    responses(
        (status = 200,body=ResObj<Vec<DeptList>>,description ="部门列表")
    ),
)]
pub async fn get_dept_list(req:&mut Request) ->Res<Vec<DeptList>>{
    let payload = req.parse_queries::<DeptListPayload>().map_or(DeptListPayload{ dept_name: None, status: None },|v|v);
    match_ok(dept_service::get_dept_list(payload.dept_name,payload.status).await)
}