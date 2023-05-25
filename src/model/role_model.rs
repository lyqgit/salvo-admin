use salvo::oapi::{ToParameters, ToSchema};
use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[serde(rename_all(serialize="camelCase"))]
#[schema(rename_all="camelCase")]
pub struct SysRoleList{
    pub role_id:Option<i64>,
    pub role_name:Option<String>,
    pub role_key:Option<String>,
    pub role_sort:Option<i64>,
    pub data_scope:Option<String>,
    pub menu_check_strictly:Option<i8>,
    pub dept_check_strictly:Option<i8>,
    pub status:Option<String>,
    pub del_flag:Option<String>,
    pub create_by:Option<String>,
    pub create_time:Option<DateTime>,
    pub update_by:Option<String>,
    pub update_time:Option<DateTime>,
    pub remark:Option<String>,
}

#[derive(Debug,Serialize,Deserialize,Clone,ToParameters)]
#[serde(rename_all(deserialize ="camelCase"))]
#[parameters(rename_all="camelCase")]
#[parameters(parameter_in = Query)]
pub struct SysRolePagePayload{
    pub page_num:Option<u64>,
    pub page_size:Option<u64>,
    pub role_name:Option<String>,
    pub role_key:Option<String>,
    #[serde(rename(deserialize ="params[beginTime]"))]
    #[parameter(rename="params[beginTime]")]
    #[parameter(value_type = Option<String>)]
    pub begin_time:Option<DateTime>,
    #[serde(rename(deserialize ="params[endTime]"))]
    #[parameter(rename="params[endTime]")]
    #[parameter(value_type = Option<String>)]
    pub end_time:Option<DateTime>,
}
