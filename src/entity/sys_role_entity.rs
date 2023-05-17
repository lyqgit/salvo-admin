use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
pub struct SysUser{
  #[serde(rename(serialize = "roleId"))]
  pub role_id:i64,
}