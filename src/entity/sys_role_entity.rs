use serde::{Serialize, Deserialize};
use rbatis::rbdc::datetime::DateTime;
use crate::model::role_model::SysRoleList;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SysRole{
  pub role_id:i64,
  pub role_name:String,
  pub role_key:String,
  pub role_sort:i64,
  pub data_scope:String,
  pub menu_check_strictly:i8,
  pub dept_check_strictly:i8,
  pub status:String,
  pub del_flag:String,
  pub create_by:String,
  pub create_time:DateTime,
  pub update_by:Option<String>,
  pub update_time:Option<DateTime>,
  pub remark:Option<String>,
}

impl SysRole {
  #[allow(dead_code)]
  #[allow(unused_must_use)]
  pub fn into_sys_role_list(self)->SysRoleList{
    SysRoleList{
      role_id: Some(self.role_id),
      role_name: Some(self.role_name),
      role_key: Some(self.role_key),
      role_sort: Some(self.role_sort),
      data_scope: Some(self.data_scope),
      menu_check_strictly: Some(self.menu_check_strictly),
      dept_check_strictly: Some(self.dept_check_strictly),
      status: Some(self.status),
      del_flag: Some(self.del_flag),
      create_by: Some(self.create_by),
      create_time: Some(self.create_time),
      update_by: self.update_by,
      update_time: self.update_time,
      remark: self.remark
    }
  }
}