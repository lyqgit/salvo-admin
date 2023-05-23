use salvo::{oapi::{ToSchema,ToParameters,Required}};
use serde::{Serialize,Deserialize};
use crate::entity::sys_dict_data_entity::{SysDictDataEntity,SysDictData};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,ToSchema,Deserialize,Clone)]
#[schema(rename_all="camelCase")]
pub struct AddDictType{
  #[serde(rename(deserialize="dictName"))]
  pub dict_name:String,
  #[serde(rename(deserialize="dictType"))]
  pub dict_type:String,
  pub status:String,
  pub remark:Option<String>
}

#[derive(Debug,Serialize,Deserialize,Clone,ToParameters)]
#[parameters(rename_all="camelCase")]
#[parameters(parameter_in = Query)]
pub struct DictTypePagePayload{
  pub page_num:u64,
  pub page_size:u64,
  #[parameter(value_type = Option<String>)]
  pub dict_name:String,
  #[parameter(value_type = Option<String>)]
  pub dict_type:String,
  #[parameter(value_type = Option<String>)]
  pub status:Option<String>,
  #[parameter(rename="params[beginTime]")]
  #[serde(rename(deserialize="params[beginTime]"))]
  #[parameter(value_type = Option<String>)]
  pub begin_time:DateTime,
  #[parameter(rename="params[endTime]")]
  #[serde(rename(deserialize="params[endTime]"))]
  #[parameter(value_type = Option<String>)]
  pub end_time:DateTime,
}


#[derive(Debug,Serialize,ToSchema,Deserialize,Clone,ToParameters)]
#[parameters(rename_all="camelCase")]
pub struct DictTypeDataPagePayload{
  #[serde(rename(deserialize="pageNum"))]
  pub page_num:u64,
  #[serde(rename(deserialize="pageSize"))]
  pub page_size:u64,
  #[serde(rename(deserialize="dictType"))]
  pub dict_type:String,
  pub status:Option<String>,
  #[serde(rename(deserialize="dictLabel"))]
  pub dict_label:Option<String>,
}


#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[schema(rename_all="camelCase")]
pub struct SysDictDataVo{
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
  pub remark:Option<String>
}

impl From<SysDictData> for SysDictDataVo{
  fn from(s:SysDictData)->Self{
    Self{
      dict_code:s.dict_code,
      dict_sort:s.dict_sort,
      dict_label:s.dict_label,
      dict_value:s.dict_value,
      css_class:s.css_class,
      list_class:s.list_class,
      is_default:s.is_default,
      status:s.status,
      create_by:s.create_by,
      create_time:s.create_time,
      update_by:s.update_by,
      update_time:s.update_time,
      remark:s.remark,
    }
  }
}



#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[schema(rename_all="camelCase")]
pub struct AddSysDictDataVo{
  #[serde(rename(deserialize="dictSort"))]
  pub dict_sort:i8,
  #[serde(rename(deserialize="dictLabel"))]
  pub dict_label:String,
  #[serde(rename(deserialize="dictValue"))]
  pub dict_value:String,
  #[serde(rename(deserialize="dictType"))]
  pub dict_type:String,
  #[serde(rename(deserialize="cssClass"))]
  pub css_class:Option<String>,
  #[serde(rename(deserialize="listClass"))]
  pub list_class:Option<String>,
  pub status:String,
  pub remark:Option<String>
}


impl From<AddSysDictDataVo> for SysDictDataEntity{
  fn from(s:AddSysDictDataVo)->Self{
    Self{
      dict_code:0,
      dict_sort:s.dict_sort,
      dict_label:s.dict_label,
      dict_value:s.dict_value,
      dict_type:s.dict_type,
      css_class:s.css_class,
      list_class:s.list_class,
      is_default:String::from(""),
      status:s.status,
      create_by:String::from(""),
      create_time:DateTime::now(),
      update_by:String::from(""),
      update_time:None,
      remark:s.remark,
    }
  }
}
