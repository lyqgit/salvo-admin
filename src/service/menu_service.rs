use rbatis::rbdc::datetime::DateTime;
use crate::mapper::{menu_mapper, role_menu_mapper};
use crate::GLOBAL_DB;
use crate::entity::sys_menu_entity::SysMenu;
use crate::entity::sys_user_entity::SysUser;
use crate::model::menu_model::{MenuTree, Router, SysMenuModifyPayload, SysMenuPage};
use crate::utils::func;

pub async fn get_menu_by_role_id(is_admin:bool,id:String)->rbatis::Result<Vec<String>>{
  let list:Vec<SysMenu> = menu_mapper::select_menus_by_role_id(&mut GLOBAL_DB.clone(),is_admin,id).await?;
  let mut menu_list = Vec::new();
  for (_,it) in list.iter().enumerate(){
    let temp_perms = it.perms.clone().map_or(String::from(""), |v|v);
    if !temp_perms.is_empty(){
      menu_list.push(temp_perms);
    }
  }
  Ok(menu_list)
}

pub async fn get_router_tree(is_admin:bool,id:i32)->rbatis::Result<Vec<Router>>{
  let list:Vec<SysMenu> = menu_mapper::select_menus_by_user_id(&mut GLOBAL_DB.clone(),is_admin,id).await?;
  let router_list = func::router_arr_to_tree2(list,0);
  Ok(router_list)
}

pub async fn get_menu_list(menu_name:Option<String>,status:Option<String>)->rbatis::Result<Vec<SysMenuPage>>{
  let list:Vec<SysMenuPage> = menu_mapper::select_menus_list(&mut GLOBAL_DB.clone(),menu_name,status).await?;
  Ok(list)
}

pub async fn get_menu_tree()->rbatis::Result<Vec<MenuTree>>{
  let list:Vec<SysMenuPage> = menu_mapper::select_menus_list(&mut GLOBAL_DB.clone(),None,None).await?;
  let mut menu_tree_list = Vec::<MenuTree>::new();
  func::menu_arr_to_tree(&mut menu_tree_list,list,0);
  Ok(menu_tree_list)
}

pub async fn add_menu(user_id:i32,payload:SysMenuModifyPayload)->rbatis::Result<bool>{
  let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
  let user = user.get(0).unwrap();
  let mut sys_menu_entity:SysMenu = payload.into();
  sys_menu_entity.create_by = user.user_name.clone();
  let rows = SysMenu::insert(&mut GLOBAL_DB.clone(),&sys_menu_entity).await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn del_menu_by_id(menu_id:i64)->rbatis::Result<bool>{
  let rows = SysMenu::delete_by_column(&mut GLOBAL_DB.clone(),"menu_id",menu_id).await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn get_menu_detail_by_id(menu_id:i64)->rbatis::Result<Option<SysMenuPage>>{
  let list:Vec<SysMenuPage> = menu_mapper::select_menus_by_id(&mut GLOBAL_DB.clone(),menu_id).await?;
  let one = list.get(0).cloned();
  Ok(one)
}

pub async fn edit_menu(user_id:i32,payload:SysMenuModifyPayload)->rbatis::Result<bool>{
  let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
  let user = user.get(0).unwrap();
  let mut sys_menu_entity:SysMenu = payload.into();
  sys_menu_entity.update_by = user.user_name.clone();
  sys_menu_entity.update_time = Some(DateTime::now());
  let rows = SysMenu::update_by_column(&mut GLOBAL_DB.clone(),&sys_menu_entity,"menu_id").await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn get_menu_id_by_role_id(user_id:i32)->rbatis::Result<Vec<i64>>{
  let list = role_menu_mapper::get_menu_id_by_role_id(&mut GLOBAL_DB.clone(),user_id).await?;
  let list = list.into_iter().map(|v|v.menu_id).collect();
  Ok(list)
}
