use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;
use salvo::oapi::{ToParameters, ToSchema};

#[derive(Debug,Serialize,Deserialize,ToSchema,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct SysNoticeList{
    pub notice_id:Option<i64>,
    pub notice_title:Option<String>,
    pub notice_type:Option<String>,
    pub notice_content:Option<String>,
    pub status:Option<String>,
    pub create_by:Option<String>,
    pub create_time:Option<DateTime>,
    pub update_by:Option<String>,
    pub update_time:Option<DateTime>,
    pub remark:Option<String>,
}

#[derive(Debug,Serialize,Deserialize,ToParameters,Clone)]
#[salvo(parameters(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
#[salvo(parameters(default_parameter_in = Query))]
pub struct SysNoticeListPayload{
    pub page_num:u64,
    pub page_size:u64,
    pub notice_title:Option<String>,
    pub notice_type:Option<String>,
    pub status:Option<String>,
}

#[derive(Debug,Serialize,Deserialize,ToSchema,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
pub struct SysNoticeModifyPayload{
    pub notice_id:Option<i64>,
    pub notice_title:Option<String>,
    pub notice_type:Option<String>,
    pub notice_content:Option<String>,
    pub status:Option<String>,
    pub remark:Option<String>,
}
