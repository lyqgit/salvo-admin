use salvo::{Depot, oapi::endpoint, Request};
use salvo::oapi::extract::{JsonBody, PathParam};
use crate::model::menu_model::{MenuTree, RoleMenuTree, SysMenuModifyPayload, SysMenuPage, SysMenuPagePayload};
use crate::service::menu_service;
use crate::utils::res::{match_ok, Res, res_json_custom, res_json_ok, ResObj};

/// 菜单列表
#[endpoint(
    tags("菜单"),
    parameters(
        SysMenuPagePayload
    ),
    responses(
        (status_code = 200,body=ResObj<Vec<SysMenuPage>>,description ="菜单列表")
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

/// 添加菜单
#[endpoint(
    tags("菜单"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="添加菜单")
    ),
)]
pub async fn add_menu(payload:JsonBody<SysMenuModifyPayload>,depot:&mut Depot)->Res<()>{
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    match menu_service::add_menu(user_id,payload.into_inner()).await {
        Ok(v)=>{
            if v {
                Ok(res_json_ok(Some(())))
            }else{
                Err(res_json_custom(400,"添加失败".to_string()))
            }
        },
        Err(err)=>{
            Err(res_json_custom(400,err.to_string()))
        }
    }
}

/// 删除菜单
#[endpoint(
    tags("菜单"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="删除菜单")
    ),
)]
pub async fn del_menu_by_id(id:PathParam<i64>)->Res<()>{
    match menu_service::del_menu_by_id(id.into_inner()).await{
        Ok(v)=>{
            if v {
                Ok(res_json_ok(Some(())))
            }else{
                Err(res_json_custom(400,"删除失败".to_string()))
            }
        },
        Err(err)=>{
            Err(res_json_custom(400,err.to_string()))
        }
    }
}

/// 菜单详情
#[endpoint(
    tags("菜单"),
    responses(
        (status_code = 200,body=ResObj<SysMenuPage>,description ="菜单详情")
    ),
)]
pub async fn get_menu_by_id(id:PathParam<i64>)->Res<SysMenuPage>{
    match menu_service::get_menu_detail_by_id(id.into_inner()).await {
        Ok(v)=>{
            Ok(res_json_ok(v))
        },
        Err(err)=>{
            Err(res_json_custom(400,err.to_string()))
        }
    }
}

/// 修改菜单
#[endpoint(
    tags("菜单"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="修改菜单")
    ),
)]
pub async fn put_edit_menu(payload:JsonBody<SysMenuModifyPayload>,depot:&mut Depot)->Res<()>{
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    match menu_service::edit_menu(user_id,payload.into_inner()).await {
        Ok(v)=>{
            if v {
                Ok(res_json_ok(Some(())))
            }else{
                Err(res_json_custom(400,"修改失败".to_string()))
            }
        },
        Err(err)=>{
            Err(res_json_custom(400,err.to_string()))
        }
    }
}

/// 菜单树
#[endpoint(
    tags("菜单"),
    responses(
        (status_code = 200,body=ResObj<Vec<MenuTree>>,description ="菜单树")
    ),
)]
pub async fn get_menu_tree()->Res<Vec<MenuTree>>{
    match_ok(menu_service::get_menu_tree().await)
}

/// 菜单树和根据用户获取已选菜单id
#[endpoint(
    tags("菜单"),
    responses(
        (status_code = 200,body=ResObj<RoleMenuTree>,description ="菜单树和根据用户获取已选菜单id")
    ),
)]
pub async fn get_role_menu_tree_by_user_id(id:PathParam<i32>)->Res<RoleMenuTree>{
    match menu_service::get_menu_tree().await{
        Ok(v1)=>{
            match menu_service::get_menu_id_by_role_id(id.into_inner()).await {
                Ok(v2)=>{
                    let role_menu_tree = RoleMenuTree{
                        menus:v1,
                        checked_keys:v2
                    };
                    Ok(res_json_ok(Some(role_menu_tree)))
                },
                Err(err)=>{
                    Err(res_json_custom(400,err.to_string()))
                }
            }
        },
        Err(err)=>{
            Err(res_json_custom(400,err.to_string()))
        }
    }
}