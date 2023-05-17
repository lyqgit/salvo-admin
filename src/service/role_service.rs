use crate::mapper::role_mapper;
use crate::GLOBAL_DB;

pub async fn get_roles_by_user_id(id:i32)->rbatis::Result<Vec<String>>{
  let list = role_mapper::select_roles_by_user_id(&mut GLOBAL_DB.clone(),id).await?;
  Ok(list)
}
