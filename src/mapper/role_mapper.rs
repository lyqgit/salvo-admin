use rbatis::{html_sql, executor::Executor,crud};
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use crate::entity::sys_role_entity::SysRole;
use crate::model::role_model::{SysRoleList, SysRoleListFlag};
use crate::model::user_model::SysUserList;

#[html_sql("src/mapper/xml/role_xml.html")]
pub async fn select_roles_by_user_id(rb: &mut dyn Executor,id:i32)->rbatis::Result<Vec<SysRole>>{
  impled!()
}

#[html_sql("src/mapper/xml/role_xml.html")]
pub async fn select_roles_by_page(rb: &mut dyn Executor,page_num:u64,page_size:u64,role_name:Option<String>,role_key:Option<String>,status:Option<String>,begin_time:Option<DateTime>,end_time:Option<DateTime>)->rbatis::Result<Vec<SysRoleList>>{
  impled!()
}

#[html_sql("src/mapper/xml/role_xml.html")]
pub async fn select_roles_count(rb: &mut dyn Executor,role_name:Option<String>,role_key:Option<String>,status:Option<String>,begin_time:Option<DateTime>,end_time:Option<DateTime>)->rbatis::Result<u64>{
  impled!()
}

#[html_sql("src/mapper/xml/role_xml.html")]
pub async fn update_role_status_by_id(rb: &mut dyn Executor,role_id:i64,status:String)->rbatis::Result<ExecResult>{
  impled!()
}

#[html_sql("src/mapper/xml/role_xml.html")]
pub async fn del_role_by_id(rb: &mut dyn Executor,role_id:String)->rbatis::Result<ExecResult>{
  impled!()
}

#[html_sql("src/mapper/xml/role_xml.html")]
pub async fn get_role_by_id(rb: &mut dyn Executor,role_id:String)->rbatis::Result<Vec<SysRoleList>>{
  impled!()
}

#[html_sql("src/mapper/xml/role_xml.html")]
pub async fn select_roles_list(rb: &mut dyn Executor)->rbatis::Result<Vec<SysRoleList>>{
  impled!()
}

#[html_sql("src/mapper/xml/role_xml.html")]
pub async fn select_roles_list_flag_and_status(rb: &mut dyn Executor)->rbatis::Result<Vec<SysRoleList>>{
  impled!()
}

#[html_sql("src/mapper/xml/role_xml.html")]
pub async fn select_roles_list_and_is_flag(rb: &mut dyn Executor)->rbatis::Result<Vec<SysRoleListFlag>>{
  impled!()
}

#[html_sql("src/mapper/xml/role_xml.html")]
pub async fn select_roles_list_by_auth_id(rb: &mut dyn Executor,user_name:Option<String>,phone_number:Option<String>,role_id:i64,page_num:u64,page_size:u64)->rbatis::Result<Vec<SysUserList>>{
  impled!()
}

#[html_sql("src/mapper/xml/role_xml.html")]
pub async fn select_count_roles_list_by_auth_id(rb: &mut dyn Executor,user_name:Option<String>,phone_number:Option<String>,role_id:i64)->rbatis::Result<u64>{
  impled!()
}

#[html_sql("src/mapper/xml/role_xml.html")]
pub async fn select_roles_list_not_in_role_id(rb: &mut dyn Executor,user_name:Option<String>,phone_number:Option<String>,role_id:i64,page_num:u64,page_size:u64)->rbatis::Result<Vec<SysUserList>>{
  impled!()
}

#[html_sql("src/mapper/xml/role_xml.html")]
pub async fn select_count_roles_list_not_in_role_id(rb: &mut dyn Executor,user_name:Option<String>,phone_number:Option<String>,role_id:i64)->rbatis::Result<u64>{
  impled!()
}

crud!(SysRole{});
