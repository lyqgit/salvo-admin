use rbatis::rbdc::datetime::DateTime;
use crate::mapper::{role_mapper, role_menu_mapper, user_role_mapper};
use crate::GLOBAL_DB;
use crate::entity::sys_role_entity::SysRole;
use crate::entity::sys_role_menu_entity::SysRoleMenuEntity;
use crate::entity::sys_user_role_entity::SysUserRoleEntity;
use crate::entity::sys_user_entity::SysUser;
use crate::model::role_model::SysRoleList;
use crate::utils::func;
use crate::model::common_model::Page;
use crate::model::user_model::SysUserList;
use crate::utils::func::is_modify_ok;

pub async fn get_roles_by_user_id(id:i32)->rbatis::Result<Vec<String>>{
  let list:Vec<SysRole> = role_mapper::select_roles_by_user_id(&mut GLOBAL_DB.clone(),id).await?;
  let mut role_list = Vec::new();
  for (_,it) in list.iter().enumerate(){
    role_list.push(it.role_key.clone());
  }
  Ok(role_list)
}

pub async fn get_role_by_page(page_num:u64,page_size:u64,role_name:Option<String>,role_key:Option<String>,status:Option<String>,begin_time:Option<DateTime>,end_time:Option<DateTime>)->rbatis::Result<Page<SysRoleList>>{
  let (num,size) = func::create_page(page_num,page_size);
  let list:Vec<SysRoleList> = role_mapper::select_roles_by_page(&mut GLOBAL_DB.clone(),num,size,role_name.clone(),role_key.clone(),status.clone(),begin_time.clone(),end_time.clone()).await?;
  let count = role_mapper::select_roles_count(&mut GLOBAL_DB.clone(),role_name.clone(),role_key.clone(),status.clone(),begin_time,end_time).await?;
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
  let mut tx = GLOBAL_DB.acquire_begin().await.unwrap();
  let row = SysRole::insert(&mut GLOBAL_DB.clone(),&sys_role).await?;
  if menu_ids.len()>0{
    let mut sys_role_user_entity = Vec::<SysRoleMenuEntity>::new();
    for (_,it) in menu_ids.iter().enumerate(){
      let temp = SysRoleMenuEntity{
        role_id: i64::from(row.last_insert_id.clone()),
        menu_id: it.clone(),
      };
      sys_role_user_entity.push(temp);
    }
    let row = SysRoleMenuEntity::insert_batch(&mut GLOBAL_DB.clone(), &sys_role_user_entity, sys_role_user_entity.len() as u64).await?;
    tx.commit().await.unwrap();
    tx.rollback().await.unwrap();
    Ok(func::is_modify_ok(row.rows_affected))
  }else{
    tx.commit().await.unwrap();
    tx.rollback().await.unwrap();
    Ok(func::is_modify_ok(row.rows_affected))
  }

}

pub async fn update_role_status_by_id(role_id:i64,status:String)->rbatis::Result<bool>{
  let rows = role_mapper::update_role_status_by_id(&mut GLOBAL_DB.clone(),role_id,status).await?;
  Ok(is_modify_ok(rows.rows_affected))
}

pub async fn del_role_by_id(role_id:String)->rbatis::Result<bool>{
  let mut tx = GLOBAL_DB.acquire_begin().await.unwrap();
  role_mapper::del_role_by_id(&mut GLOBAL_DB.clone(),role_id.clone()).await?;
  let rows = role_menu_mapper::del_role_menu_by_role_id(&mut GLOBAL_DB.clone(),role_id).await?;
  tx.commit().await.unwrap();
  tx.rollback().await.unwrap();
  Ok(is_modify_ok(rows.rows_affected))
}

pub async fn get_role_by_id(role_id:String)->rbatis::Result<Option<SysRoleList>>{
  let list = role_mapper::get_role_by_id(&mut GLOBAL_DB.clone(),role_id).await?;
  let one = list.get(0).cloned();
  Ok(one)
}



pub async fn edit_role_and_bind_menu(
  user_id:i32,
  menu_check_strictly:bool,
  dept_check_strictly:bool,
  menu_ids:Vec<i64>,
  role_key:String,
  role_name:String,
  status:String,
  data_scope:String,
  del_flag:String,
  create_by:String,
  create_time:DateTime,
  role_sort:i64,
  remark:Option<String>,
  role_id:i64,
) ->rbatis::Result<bool>{
  let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
  let user = user.get(0).unwrap();
  let sys_role = SysRole{
    role_id,
    role_name,
    role_key,
    role_sort,
    data_scope,
    menu_check_strictly: menu_check_strictly.into(),
    dept_check_strictly: dept_check_strictly.into(),
    status,
    remark,
    create_by:create_by,
    create_time,
    update_by:Some(user.user_name.clone()),
    update_time:Some(DateTime::now()),
    del_flag,
  };
  let mut tx = GLOBAL_DB.acquire_begin().await.unwrap();
  let row = SysRole::update_by_column(&mut GLOBAL_DB.clone(),&sys_role,"role_id").await?;
  if menu_ids.len()>0{
    SysRoleMenuEntity::delete_by_column(&mut GLOBAL_DB.clone(),"role_id",role_id).await?;
    let mut sys_role_user_entity = Vec::<SysRoleMenuEntity>::new();
    for (_,it) in menu_ids.iter().enumerate(){
      let temp = SysRoleMenuEntity{
        role_id,
        menu_id: it.clone(),
      };
      sys_role_user_entity.push(temp);
    }
    let row = SysRoleMenuEntity::insert_batch(&mut GLOBAL_DB.clone(), &sys_role_user_entity, sys_role_user_entity.len() as u64).await?;
    tx.commit().await.unwrap();
    tx.rollback().await.unwrap();
    Ok(func::is_modify_ok(row.rows_affected))
  }else{
    tx.commit().await.unwrap();
    tx.rollback().await.unwrap();
    Ok(func::is_modify_ok(row.rows_affected))
  }

}

pub async fn select_users_by_role_id(user_name:Option<String>,phone_number:Option<String>,role_id:i64,page_num:u64,page_size:u64)->rbatis::Result<Page<SysUserList>>{
  let (num,size) = func::create_page(page_num,page_size);
  let list = role_mapper::select_roles_list_by_auth_id(&mut GLOBAL_DB.clone(),user_name.clone(),phone_number.clone(),role_id,num,size).await?;
  let total = role_mapper::select_count_roles_list_by_auth_id(&mut GLOBAL_DB.clone(),user_name,phone_number,role_id).await?;
  Ok(Page{rows:list,total})
}

pub async fn select_users_not_in_role_id(user_name:Option<String>,phone_number:Option<String>,role_id:i64,page_num:u64,page_size:u64)->rbatis::Result<Page<SysUserList>>{
  let (num,size) = func::create_page(page_num,page_size);
  let list = role_mapper::select_roles_list_not_in_role_id(&mut GLOBAL_DB.clone(),user_name.clone(),phone_number.clone(),role_id,num,size).await?;
  let total = role_mapper::select_count_roles_list_not_in_role_id(&mut GLOBAL_DB.clone(),user_name,phone_number,role_id).await?;
  Ok(Page{rows:list,total})
}

pub async fn del_user_role_bind_more(user_id:String,role_id:i64)->rbatis::Result<bool>{
  let rows = user_role_mapper::del_by_role_and_user_id_more(&mut GLOBAL_DB.clone(),user_id,role_id).await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn del_user_role_bind(user_id:i64,role_id:String)->rbatis::Result<bool>{
  let rows = user_role_mapper::del_by_role_and_user_id(&mut GLOBAL_DB.clone(),user_id,role_id).await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn bind_more_user_and_simple_role(user_ids:String,role_id:i64)->rbatis::Result<bool>{
  let user_arr_str:Vec<&str> = user_ids.split(",").collect::<Vec<&str>>();
  let user_id_arr = user_arr_str.into_iter().map(|it|it.parse::<i64>().unwrap()).collect::<Vec<i64>>();
  let mut user_role_arr = Vec::<SysUserRoleEntity>::new();
  for it in user_id_arr.iter(){
    let user_role = SysUserRoleEntity{
      user_id:it.clone(),
      role_id
    };
    user_role_arr.push(user_role);
  }
  let mut tx = GLOBAL_DB.acquire_begin().await?;
  let rows = SysUserRoleEntity::insert_batch(&mut GLOBAL_DB.clone(), &user_role_arr, user_role_arr.len() as u64).await?;
  tx.commit().await.unwrap();
  tx.rollback().await.unwrap();
  Ok(func::is_modify_ok(rows.rows_affected))
}