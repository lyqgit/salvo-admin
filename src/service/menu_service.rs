use crate::mapper::menu_mapper;
use crate::GLOBAL_DB;
use crate::entity::sys_menu_entity::SysMenu;

pub async fn get_menu_by_role_id(is_admin:bool,id:String)->rbatis::Result<Vec<String>>{
  let list:Vec<SysMenu> = menu_mapper::select_menus_by_role_id(&mut GLOBAL_DB.clone(),is_admin,id).await?;
  let mut menu_list = Vec::new();
  for (_,it) in list.iter().enumerate(){
    menu_list.push(it.perms.clone().map_or(String::from(""), |v|v));
  }
  Ok(menu_list)
}

pub async fn get_router_tree(is_admin:bool,id:i32){
  let list:Vec<SysMenu> = menu_mapper::select_menus_by_role_id(&mut GLOBAL_DB.clone(),is_admin,id).await?;
  
}
