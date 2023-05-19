use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
pub struct SysDictType{
  #[serde(rename(serialize="dictId"))]
  pub dict_id:i64,
  #[serde(rename(serialize="dictName"))]
  pub dict_name:String,
  #[serde(rename(serialize="dictType"))]
  pub dict_type:String,
  pub status:String,
  #[serde(rename(serialize="createBy"))]
  pub create_by:String,
  #[serde(rename(serialize="createTime"))]
  pub create_time:DateTime,
  #[serde(rename(serialize="updateBy"))]
  pub update_by:Option<String>,
  #[serde(rename(serialize="updateTime"))]
  pub update_time:Option<DateTime>,
  remark:Option<String>
}