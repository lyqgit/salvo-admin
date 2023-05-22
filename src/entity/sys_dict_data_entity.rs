use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
pub struct SysDictData{
  #[serde(rename(serialize="dictCode"))]
  pub dict_code:i64,
  #[serde(rename(serialize="dictSort"))]
  pub dict_sort:i8,
  #[serde(rename(serialize="dictLabel"))]
  pub dict_label:String,
  #[serde(rename(serialize="dictValue"))]
  pub dict_value:String,
  #[serde(rename(serialize="cssClass"))]
  pub css_class:Option<String>,
  #[serde(rename(serialize="listClass"))]
  pub list_class:Option<String>,
  #[serde(rename(serialize="isDefault"))]
  pub is_default:String,
  pub status:String,
  #[serde(rename(serialize="createBy"))]
  pub create_by:String,
  #[serde(rename(serialize="createTime"))]
  pub create_time:DateTime,
  #[serde(rename(serialize="updateBy"))]
  pub update_by:String,
  #[serde(rename(serialize="updateTime"))]
  pub update_time:Option<DateTime>,
  remark:Option<String>
}