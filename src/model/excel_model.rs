use salvo::{oapi::{ToSchema}};
use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,ToSchema,Deserialize,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
pub struct ExcelModifyPayload{
    pub excel_id:Option<String>,
    pub excel_name:Option<String>,
    pub excel_data:Option<String>,
}

#[derive(Debug,Serialize,ToSchema,Deserialize,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct ExcelList{
    pub excel_id:Option<String>,
    pub excel_name:Option<String>,
}

#[derive(Debug,Serialize,ToSchema,Deserialize,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct ExcelDetail{
    pub excel_id:Option<String>,
    pub excel_name:Option<String>,
    pub excel_data:Option<String>,
    pub create_time:Option<DateTime>,
    pub update_time:Option<DateTime>,
}
