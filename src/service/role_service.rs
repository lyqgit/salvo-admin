use rbatis::rbdc::datetime::DateTime;
use crate::mapper::role_mapper;
use crate::GLOBAL_DB;
use crate::entity::sys_role_entity::SysRole;
use crate::entity::sys_role_menu_entity::SysRoleUserEntity;
use crate::entity::sys_user_entity::SysUser;
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

pub async fn add_role_and_bind_menu(
  user_id:i32,
  menu_check_strictly:bool,
  dept_check_strictly:bool,
  menu_ids:Vec<i64>,
  role_key:String,
  role_name:String,
  status:String,
  role_sort:i64,
  remark:Option<String>,
) ->rbatis::Result<bool>{
  let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
  let user = user.get(0).unwrap();
  let sys_role = SysRole{
    role_id:0,
    role_name,
    role_key,
    role_sort,
    data_scope: "2".to_string(),
    menu_check_strictly: menu_check_strictly.into(),
    dept_check_strictly: dept_check_strictly.into(),
    status,
    remark,
    create_by:user.user_name.clone(),
    create_time:DateTime::now(),
    update_by:None,
    update_time:None,
    del_flag: "0".to_string(),
  };
  let row = SysRole::insert(&mut GLOBAL_DB.clone(),&sys_role).await?;
  let mut sys_role_user_entity = Vec::<SysRoleUserEntity>::new();
  for (_,it) in menu_ids.iter().enumerate(){
    let temp = SysRoleUserEntity{
      role_id: i64::from(row.last_insert_id.clone()),
      menu_id: it.clone(),
    };
    sys_role_user_entity.push(temp);
  }
  let row = SysRoleUserEntity::insert_batch(&mut GLOBAL_DB.clone(), &sys_role_user_entity, sys_role_user_entity.len() as u64).await?;
  Ok(func::is_modify_ok(row.rows_affected))
}
