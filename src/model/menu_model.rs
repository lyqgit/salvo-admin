use salvo::oapi::ToSchema;
use serde::{Serialize};

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
