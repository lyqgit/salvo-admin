use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;
use salvo::oapi::{ToParameters, ToSchema};

#[derive(Debug,Serialize,Deserialize,ToSchema,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct SysPostList{
    pub post_id:Option<i64>,
    pub post_code:Option<String>,
    pub post_name:Option<String>,
    pub post_sort:Option<i8>,
    pub status:Option<String>,
    pub create_by:Option<String>,
    pub create_time:Option<DateTime>,
    pub update_by:Option<String>,
    pub update_time:Option<DateTime>,
}

#[derive(Debug,Serialize,Deserialize,ToParameters,Clone)]
#[salvo(parameters(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
#[salvo(parameters(default_parameter_in = Query))]
pub struct SysPostListPayload{
    pub page_num:u64,
    pub page_size:u64,
    pub post_code:Option<String>,
    pub post_name:Option<String>,
    pub status:Option<String>,
}

#[derive(Debug,Serialize,Deserialize,ToSchema,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
pub struct SysPostModifyPayload{
    pub post_id:Option<i64>,
    pub post_code:Option<String>,
    pub post_name:Option<String>,
    pub post_sort:Option<i8>,
    pub status:Option<String>,
    pub remark:Option<String>,
}
