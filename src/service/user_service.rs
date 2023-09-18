use rbatis::rbdc::datetime::DateTime;
use crate::entity::sys_user_entity::{SysUser, SysUserEntity,AddSysUserEntity};
use crate::entity::sys_captcha_entity::SysCaptcha;
use crate::GLOBAL_DB;
use rbatis::rbdc::Error;
use crate::entity::sys_user_post_entity::SysUserPostEntity;
use crate::entity::sys_user_role_entity::SysUserRoleEntity;
use crate::mapper::{post_mapper, role_mapper, user_mapper};
use crate::model::common_model::Page;
use crate::model::user_model::{SysUserAuthRole, SysUserDetail, SysUserList};
use crate::service::dept_service;
use crate::utils::func;
use crate::utils::func::{create_page, create_page_list};

pub async fn get_user_by_up(username:String,password:String)->rbatis::Result<Option<SysUser>>{
  let list = SysUser::select_user_by_up(&mut GLOBAL_DB.clone(), username, password).await?;
  let one = list.get(0).cloned();
  Ok(one)
}

#[allow(dead_code)]
pub async fn get_captcha_by_code(code:String)->Result<Option<SysCaptcha>,Error>{
  let list = SysCaptcha::select_captcha_by_code(&mut GLOBAL_DB.clone(), code).await?;
  let one = list.get(0).cloned();
  Ok(one)
}


pub async fn get_user_by_id(id:i32)->Result<Option<SysUser>,Error>{
  let list = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", id).await?;
  let one = list.get(0).cloned();
  Ok(one)
}

pub async fn get_user_page(page_num:u64,page_size:u64,user_name:Option<String>,phone_number:Option<String>,status:Option<String>,begin_time:Option<DateTime>,end_time:Option<DateTime>,dept_id:Option<i64>)->rbatis::Result<Page<SysUserList>>{
  let (num,size) = create_page(page_num,page_size);
  let mut list = user_mapper::get_user_page(&mut GLOBAL_DB.clone(),num,size,user_name.clone(),phone_number.clone(),status.clone(),begin_time.clone(),end_time.clone(),dept_id.clone()).await?;
  let length = list.len();
  for i in 0..length{
    let temp_dept = dept_service::get_dept_by_id(list[i].dept_id.unwrap()).await?;
    list[i].dept = temp_dept
  }
  let total = user_mapper::get_user_count(&mut GLOBAL_DB.clone(),user_name.clone(),phone_number.clone(),status.clone(),begin_time.clone(),end_time.clone(),dept_id.clone()).await?;
  Ok(create_page_list(list,total))
}

pub async fn get_detail_by_id(user_id:Option<i64>)->rbatis::Result<SysUserDetail>{
  let role_list = role_mapper::select_roles_list_flag_and_status(&mut GLOBAL_DB.clone()).await?;
  let post_list = post_mapper::get_post_list(&mut GLOBAL_DB.clone()).await?;
  if user_id.is_some(){
    let user = user_mapper::get_user_by_id(&mut GLOBAL_DB.clone(),user_id).await?;
    let user = user.get(0).cloned();
    let user_role_list = SysUserRoleEntity::select_by_column(&mut GLOBAL_DB.clone(),"user_id",user_id).await?;
    let mut roles_ids:Vec<i64> = Vec::new();
    user_role_list.iter().for_each(|it|{
      roles_ids.push(it.role_id)
    });
    let user_post_list = SysUserPostEntity::select_by_column(&mut GLOBAL_DB.clone(),"user_id",user_id).await?;
    let mut posts_ids:Vec<i64> = Vec::new();
    user_post_list.iter().for_each(|it|{
      posts_ids.push(it.post_id)
    });
    Ok(SysUserDetail{
      posts:post_list,
      roles: role_list,
      post_ids: Some(posts_ids),
      role_ids: Some(roles_ids),
      user,
    })
  }else{
    Ok(SysUserDetail{
      posts:post_list,
      roles: role_list,
      post_ids: None,
      role_ids: None,
      user:None,
    })
  }
}

pub async fn update_user_status_by_id(status:String,user_id:i64)->rbatis::Result<bool>{
  let rows = user_mapper::update_user_status_by_id(&mut GLOBAL_DB.clone(),status,user_id).await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

// 如果没有填写部门，则默认为若依科技-100
// 如果没有填写岗位，则默认为普通员工-4
// 新建用户，查重用户名
pub async fn add_user(
  user_id:i32,
  dept_id:Option<i64>,
  email:Option<String>,
  nick_name:String,
  user_name:String,
  password:String,
  status:Option<String>,
  sex:Option<String>,
  phone_number:Option<String>,
  post_ids:Vec<i64>,
  role_ids:Vec<i64>,
  remark:Option<String>,
)->rbatis::Result<bool>{

  // 查重用户名
  let has_user = SysUserEntity::select_by_column(&mut GLOBAL_DB.clone(), "user_name",&user_name).await?;
  if has_user.len() >0 {
    return Err(Error::from("用户名不能重复"))
  }

  let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
  let user = user.get(0).unwrap();
  let user_entity = AddSysUserEntity{
    user_id: None,
    dept_id: Option::from(dept_id.unwrap_or(100)),
    user_name: Some(user_name),
    nick_name: Some(nick_name),
    user_type: None,
    email,
    phone_number,
    sex,
    avatar: None,
    password: Some(password),
    status,
    del_flag: Some("0".to_string()),
    login_ip: None,
    login_date: None,
    create_by: Some(user.user_name.clone()),
    create_time: Some(DateTime::now()),
    update_by: None,
    update_time: None,
    remark,
  };
  let mut tx = GLOBAL_DB.acquire_begin().await?;

  let mut rows = AddSysUserEntity::insert(&mut GLOBAL_DB.clone(),&user_entity).await?;
  let new_user_id = rows.last_insert_id.as_i64().unwrap();
  let mut user_post_arr:Vec<SysUserPostEntity> = Vec::new();
  if post_ids.len()>0{
    for(_,it) in post_ids.iter().enumerate(){
      user_post_arr.push(
        SysUserPostEntity{ user_id: new_user_id, post_id: it.clone() }
      )
    }
  }else{
    user_post_arr.push(
      SysUserPostEntity{ user_id: new_user_id, post_id: 4 }
    )
  }

  if user_post_arr.len()>0{
    rows = SysUserPostEntity::insert_batch(&mut GLOBAL_DB.clone(), &user_post_arr, user_post_arr.len() as u64).await?;
  }
  let mut user_role_arr:Vec<SysUserRoleEntity> = Vec::new();
  for(_,it) in role_ids.iter().enumerate(){
    user_role_arr.push(
      SysUserRoleEntity{ user_id: new_user_id, role_id: it.clone() }
    )
  }
  if user_role_arr.len()>0{
    rows = SysUserRoleEntity::insert_batch(&mut GLOBAL_DB.clone(), &user_role_arr, user_role_arr.len() as u64).await?;
  }

  tx.commit().await?;
  tx.rollback().await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}



pub async fn edit_user(
  handle_user_id:i32,
  user_id:Option<i64>,
  dept_id:Option<i64>,
  email:Option<String>,
  nick_name:String,
  status:Option<String>,
  sex:Option<String>,
  phone_number:Option<String>,
  post_ids:Vec<i64>,
  role_ids:Vec<i64>,
  remark:Option<String>,
)->rbatis::Result<bool>{
  let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", handle_user_id).await?;
  let user = user.get(0).unwrap();
  let user_entity = SysUserEntity{
    user_id,
    dept_id,
    user_name: None,
    nick_name: Some(nick_name),
    user_type: None,
    email,
    phone_number,
    sex,
    avatar: None,
    password: None,
    status,
    del_flag: None,
    login_ip: None,
    login_date: None,
    create_by: None,
    create_time: None,
    update_by: Some(user.user_name.clone()),
    update_time: Some(DateTime::now()),
    remark,
    real_name: None,
    expire_time: None,
  };
  let mut tx = GLOBAL_DB.acquire_begin().await?;

  let mut rows = SysUserEntity::update_by_column(&mut GLOBAL_DB.clone(),&user_entity,"user_id").await?;
  let new_user_id = rows.last_insert_id.as_i64().unwrap();
  let mut user_post_arr:Vec<SysUserPostEntity> = Vec::new();
  for(_,it) in post_ids.iter().enumerate(){
    user_post_arr.push(
      SysUserPostEntity{ user_id: new_user_id, post_id: it.clone() }
    )
  }
  if user_post_arr.len()>0{
    SysUserPostEntity::delete_by_column(&mut GLOBAL_DB.clone(),"user_id",user_id).await?;
    rows = SysUserPostEntity::insert_batch(&mut GLOBAL_DB.clone(), &user_post_arr, user_post_arr.len() as u64).await?;
  }
  let mut user_role_arr:Vec<SysUserRoleEntity> = Vec::new();
  for(_,it) in role_ids.iter().enumerate(){
    user_role_arr.push(
      SysUserRoleEntity{ user_id: new_user_id, role_id: it.clone() }
    )
  }
  if user_role_arr.len()>0{
    SysUserRoleEntity::delete_by_column(&mut GLOBAL_DB.clone(),"user_id",user_id).await?;
    rows = SysUserRoleEntity::insert_batch(&mut GLOBAL_DB.clone(), &user_role_arr, user_role_arr.len() as u64).await?;
  }

  tx.commit().await?;
  tx.rollback().await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn del_user(user_id:i64)->rbatis::Result<bool>{
  let rows = SysUserEntity::delete_by_column(&mut GLOBAL_DB.clone(),"user_id",user_id).await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn update_user_pwd(user_id:i64,password:String)->rbatis::Result<bool>{
  let sys_user_entity = SysUserEntity{
    user_id: Some(user_id),
    dept_id: None,
    user_name: None,
    nick_name: None,
    user_type: None,
    email: None,
    phone_number: None,
    sex: None,
    avatar: None,
    password: Some(password),
    status: None,
    del_flag: None,
    login_ip: None,
    login_date: None,
    create_by: None,
    create_time: None,
    update_by: None,
    update_time: None,
    remark: None,
    real_name: None,
    expire_time: None,
  };
  let rows = SysUserEntity::update_by_column(&mut GLOBAL_DB.clone(),&sys_user_entity,"user_id").await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn get_user_auth_role_by_id(user_id:i64)->rbatis::Result<SysUserAuthRole>{
  let user = user_mapper::get_user_by_id(&mut GLOBAL_DB.clone(),Some(user_id)).await?;
  let user = user.get(0).cloned();
  let user_role_list = role_mapper::select_roles_by_user_id(&mut GLOBAL_DB.clone(), user_id as i32).await?;
  let role_id_list:Vec<i64> = user_role_list.iter().map(|it|it.role_id).collect();
  let mut roles = role_mapper::select_roles_list_and_is_flag(&mut GLOBAL_DB.clone()).await?;
  for it in roles.iter_mut(){
    if role_id_list.contains(&it.role_id.unwrap()){
      it.flag = Some(true);
    }

  }

  let sys_user_auth_role = SysUserAuthRole{
    user,
    roles
  };
  Ok(sys_user_auth_role)
}

pub async fn add_user_and_role(user_id:i64,role_id:Option<Vec<i64>>)->rbatis::Result<bool>{
  if role_id.is_some(){
    let role_arr = role_id.unwrap();
    let mut user_role_arr = Vec::<SysUserRoleEntity>::new();
    for it in role_arr.iter(){
      let user_role = SysUserRoleEntity{
        user_id,
        role_id:it.clone()
      };
      user_role_arr.push(user_role);
    }
    let mut tx = GLOBAL_DB.acquire_begin().await?;
    SysUserRoleEntity::delete_by_column(&mut GLOBAL_DB.clone(),"user_id",user_id).await?;
    let rows = SysUserRoleEntity::insert_batch(&mut GLOBAL_DB.clone(), &user_role_arr, user_role_arr.len() as u64).await?;
    tx.commit().await.unwrap();
    tx.rollback().await.unwrap();
    Ok(func::is_modify_ok(rows.rows_affected))
  }else{
    let rows = SysUserRoleEntity::delete_by_column(&mut GLOBAL_DB.clone(),"user_id",user_id).await?;
    Ok(func::is_modify_ok(rows.rows_affected))
  }

}