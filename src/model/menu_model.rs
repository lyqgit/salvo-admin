use salvo::oapi::{ToParameters, ToSchema};
use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,ToSchema,Clone)]
#[schema(rename_all="camelCase")]
pub struct Router{
  #[serde(rename(serialize = "alwaysShow"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub always_show:Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub children:Option<Vec<Router>>,
  pub component:String,
  pub hidden:bool,
  pub meta:Meta,
  pub name:String,
  pub path:String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub redirect:Option<String>,
}

#[derive(Debug,Serialize,ToSchema,Clone)]
#[schema(rename_all="camelCase")]
pub struct Meta{
  pub icon:String,
  pub link:Option<String>,
  #[serde(rename(serialize = "noCache"))]
  pub no_cache:bool,
  pub title:String,
}


#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[schema(rename_all="camelCase")]
#[serde(rename_all(serialize="camelCase"))]
pub struct SysMenuPage{
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

#[derive(Debug,Serialize,Deserialize,Clone,ToParameters)]
#[parameters(rename_all="camelCase")]
#[serde(rename_all(deserialize="camelCase"))]
#[parameters(parameter_in = Query)]
pub struct SysMenuPagePayload{
  pub menu_name:Option<String>,
  pub status:Option<String>,
}
