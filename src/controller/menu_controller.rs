use salvo::{oapi::endpoint};
use crate::model::menu_model::SysMenuPage;
use crate::service::menu_service;
use crate::utils::res::{Res, res_json_custom, res_json_ok};

#[endpoint]
pub async fn get_menu_list()->Res<Vec<SysMenuPage>>{
    match menu_service::get_menu_list().await {
        Ok(v)=>{
            Ok(res_json_ok(Some(v)))
        },
        Err(err)=>{
            Err(res_json_custom(400,err.to_string()))
        }
    }
}