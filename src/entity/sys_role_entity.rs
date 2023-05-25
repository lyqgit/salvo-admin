use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

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