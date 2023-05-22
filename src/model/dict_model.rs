use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,ToSchema,Deserialize,Clone)]
pub struct AddDictType{
  #[serde(rename(deserialize="dictName"))]
  pub dict_name:String,
  #[serde(rename(deserialize="dictType"))]
  pub dict_type:String,
  pub status:String,
  pub remark:Option<String>
}