use salvo::oapi::endpoint;
use crate::utils::res::{ match_ok_common_result_no_error, Res, ResObj};
use crate::service::monitor_service;
use crate::model::monitor_model::ServerInfo;

#[endpoint(
    tags("系统"),
    responses(
        (status_code = 200,body=ResObj<ServerInfo>,description ="部门列表")
    ),
)]
pub async fn get_server_info()->Res<ServerInfo>{
    match_ok_common_result_no_error(monitor_service::get_sys_info().await)
}

