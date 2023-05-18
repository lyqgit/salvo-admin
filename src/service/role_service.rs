use crate::mapper::role_mapper;
use crate::GLOBAL_DB;
use crate::entity::sys_role_entity::SysRole;

pub async fn get_roles_by_user_id(id:i32)->rbatis::Result<Vec<String>>{
  let list:Vec<SysRole> = role_mapper::select_roles_by_user_id(&mut GLOBAL_DB.clone(),id).await?;
  let mut role_list = Vec::new();
  for (_,it) in list.iter().enumerate(){
    role_list.push(it.role_key.clone());
  }
  Ok(role_list)
}
