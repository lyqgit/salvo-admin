use salvo::{oapi::endpoint, Request};
use crate::model::menu_model::{SysMenuPage,SysMenuPagePayload};
use crate::service::menu_service;
use crate::utils::res::{Res, res_json_custom, res_json_ok,ResObj};

#[endpoint(
    parameters(
        SysMenuPagePayload
    ),
    responses(
        (status = 200,body=ResObj<Vec<SysMenuPage>>,description ="菜单列表")
    ),
)]
pub async fn get_menu_list(req:&mut Request)->Res<Vec<SysMenuPage>>{
    let payload:SysMenuPagePayload = req.parse_queries().unwrap();
    match menu_service::get_menu_list(payload.menu_name.clone(),payload.status.clone()).await {
        Ok(v)=>{
            Ok(res_json_ok(Some(v)))
        },
        Err(err)=>{
            Err(res_json_custom(400,err.to_string()))
        }
    }
}