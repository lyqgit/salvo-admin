use rbatis::rbdc::datetime::DateTime;
use crate::entity::sys_user_entity::SysUser;
use crate::entity::sys_captcha_entity::SysCaptcha;
use crate::GLOBAL_DB;
use rbatis::rbdc::Error;
use crate::entity::sys_user_post_entity::SysUserPostEntity;
use crate::entity::sys_user_role_entity::SysUserRoleEntity;
use crate::mapper::{post_mapper, role_mapper, user_mapper};
use crate::model::common_model::Page;
use crate::model::user_model::{SysUserDetail, SysUserList};
use crate::service::dept_service;
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
  let role_list = role_mapper::select_roles_list(&mut GLOBAL_DB.clone()).await?;
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