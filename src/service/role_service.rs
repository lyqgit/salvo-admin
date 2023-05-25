use rbatis::rbdc::datetime::DateTime;
use crate::mapper::role_mapper;
use crate::GLOBAL_DB;
use crate::entity::sys_role_entity::SysRole;
use crate::model::role_model::SysRoleList;
use crate::utils::func;
use crate::model::common_model::Page;

pub async fn get_roles_by_user_id(id:i32)->rbatis::Result<Vec<String>>{
  let list:Vec<SysRole> = role_mapper::select_roles_by_user_id(&mut GLOBAL_DB.clone(),id).await?;
  let mut role_list = Vec::new();
  for (_,it) in list.iter().enumerate(){
    role_list.push(it.role_key.clone());
  }
  Ok(role_list)
}

pub async fn get_role_by_page(page_num:u64,page_size:u64,role_name:Option<String>,role_key:Option<String>,begin_time:Option<DateTime>,end_time:Option<DateTime>)->rbatis::Result<Page<SysRoleList>>{
  let (num,size) = func::create_page(page_num,page_size);
  let list:Vec<SysRoleList> = role_mapper::select_roles_by_page(&mut GLOBAL_DB.clone(),num,size,role_name.clone(),role_key.clone(),begin_time.clone(),end_time.clone()).await?;
  let count = role_mapper::select_roles_count(&mut GLOBAL_DB.clone(),role_name.clone(),role_key.clone(),begin_time,end_time).await?;
  Ok(Page{rows:list,total:count})
}
