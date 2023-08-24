use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct SysDictData{
  #[serde(rename(serialize="dictCode"))]
  pub dict_code:i64,
  #[serde(rename(serialize="dictSort"))]
  pub dict_sort:i8,
  #[serde(rename(serialize="dictLabel"))]
  pub dict_label:String,
  #[serde(rename(serialize="dictValue"))]
  pub dict_value:String,
  #[serde(rename(serialize="dictType"))]
  pub dict_type:String,
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
  pub remark:Option<String>
}


#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
pub struct SysDictDataEntity{
  pub dict_code:i64,
  pub dict_sort:i8,
  pub dict_label:String,
  pub dict_value:String,
  pub dict_type:String,
  pub css_class:Option<String>,
  pub list_class:Option<String>,
  pub is_default:String,
  pub status:String,
  pub create_by:String,
  pub create_time:DateTime,
  pub update_by:String,
  pub update_time:Option<DateTime>,
  pub remark:Option<String>
}