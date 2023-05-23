use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[schema(rename_all="camelCase")]
pub struct SysMenu{
  #[serde(rename(serialize = "roleId"))]
  pub menu_id:i64,
  pub menu_name:String,
  pub parent_id:i64,
  pub order_num:i8,
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