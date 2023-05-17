use crate::mapper::menu_mapper;
use crate::GLOBAL_DB;

pub async fn get_menu_by_role_id(is_admin:bool,id:String)->rbatis::Result<Vec<String>>{
  let list = menu_mapper::select_menus_by_role_id(&mut GLOBAL_DB.clone(),is_admin,id).await?;
  Ok(list)
}
