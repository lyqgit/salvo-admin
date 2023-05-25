use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SysMenu{
  pub menu_id:i64,
  pub menu_name:String,
  pub parent_id:i64,
  pub order_num:i64,
  pub path:String,
  pub component:Option<String>,
  pub query:Option<String>,
  pub is_frame:i8,
  pub is_cache:i8,
  pub menu_type:String,
  pub visible:String,
  pub status:String,
  pub perms:Option<String>,
  pub icon:String,
  pub create_by:String,
  pub create_time:DateTime,
  pub update_by:String,
  pub update_time:Option<DateTime>,
  pub remark:String,
}