use crate::mapper::menu_mapper;
use crate::GLOBAL_DB;
use crate::entity::sys_menu_entity::SysMenu;
use crate::model::menu_model::{Router, SysMenuPage};
use crate::utils::func;
use crate::utils::func::router_arr_to_tree;

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
  let mut router_list = Vec::<Router>::new();
  router_arr_to_tree(&mut router_list,list,0);
  Ok(router_list)
}

pub async fn get_menu_list(menu_name:Option<String>,status:Option<String>)->rbatis::Result<Vec<SysMenuPage>>{
  let list:Vec<SysMenuPage> = menu_mapper::select_menus_page(&mut GLOBAL_DB.clone(),num,size,menu_name,status).await?;
  Ok(list)
}
