use salvo::oapi::{ToParameters, ToSchema};
use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[serde(rename_all(serialize="camelCase"))]
#[salvo(schema(rename_all="camelCase"))]
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

#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[serde(rename_all(serialize="camelCase"))]
#[salvo(schema(rename_all="camelCase"))]
pub struct SysRoleListFlag{
    pub role_id:Option<i64>,
    pub role_name:Option<String>,
    pub role_key:Option<String>,
    pub role_sort:Option<i64>,
    pub data_scope:Option<String>,
    pub menu_check_strictly:Option<i8>,
    pub dept_check_strictly:Option<i8>,
    pub status:Option<String>,
    pub del_flag:Option<String>,
    pub flag:Option<bool>,
    pub create_by:Option<String>,
    pub create_time:Option<DateTime>,
    pub update_by:Option<String>,
    pub update_time:Option<DateTime>,
    pub remark:Option<String>,
}

#[derive(Debug,Serialize,Deserialize,Clone,ToParameters)]
#[serde(rename_all(deserialize ="camelCase"))]
#[salvo(parameters(rename_all="camelCase"))]
#[salvo(parameters(parameter_in = Query))]
pub struct SysRolePagePayload{
    pub page_num:Option<u64>,
    pub page_size:Option<u64>,
    pub role_name:Option<String>,
    pub role_key:Option<String>,
    pub status:Option<String>,
    #[serde(rename(deserialize ="params[beginTime]"))]
    #[salvo(parameter(rename="params[beginTime]"))]
    #[salvo(parameter(value_type = Option<String>))]
    pub begin_time:Option<DateTime>,
    #[serde(rename(deserialize ="params[endTime]"))]
    #[salvo(parameter(rename="params[endTime]"))]
    #[salvo(parameter(value_type = Option<String>))]
    pub end_time:Option<DateTime>,
}


#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[serde(rename_all(deserialize ="camelCase"))]
#[salvo(schema(rename_all="camelCase"))]
pub struct SysRoleModifyPayload{
    pub dept_check_strictly:bool,
    pub menu_check_strictly:bool,
    pub dept_ids:Option<Vec<i64>>,
    pub menu_ids:Vec<i64>,
    pub remark:Option<String>,
    pub role_key:String,
    pub role_name:String,
    pub status:String,
    pub role_sort:i64,
    pub role_id:Option<i64>,
    pub data_scope:Option<String>,
    pub del_flag:Option<String>,
    pub create_by:Option<String>,
    pub create_time:Option<DateTime>,
}

#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[serde(rename_all(deserialize ="camelCase"))]
#[salvo(schema(rename_all="camelCase"))]
pub struct SysRoleStatusPayload{
    pub role_id:i64,
    pub status:String,
}


#[derive(Debug,Serialize,Deserialize,Clone,ToParameters)]
#[salvo(parameters(rename_all="camelCase"))]
#[serde(rename_all(deserialize = "camelCase"))]
#[salvo(parameters(parameter_in = Query))]
pub struct SysUserRolePagePayload{
    pub role_id:i64,
    #[serde(rename(deserialize = "phonenumber"))]
    #[salvo(parameters(value_type=Option<String>))]
    pub phone_number:Option<String>,
    #[salvo(parameters(value_type=Option<String>))]
    pub user_name:Option<String>,
    pub page_num:Option<u64>,
    pub page_size:Option<u64>,
}

#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[serde(rename_all(deserialize ="camelCase"))]
#[salvo(schema(rename_all="camelCase"))]
pub struct SysRoleCancelUserPayload{
    pub role_id:String,
    pub user_id:i64,
}

#[derive(Debug,Serialize,Deserialize,Clone,ToParameters)]
#[salvo(parameters(rename_all="camelCase"))]
#[serde(rename_all(deserialize = "camelCase"))]
#[salvo(parameters(parameter_in = Query))]
pub struct SysUserRoleCancelPayload{
    pub role_id:i64,
    pub user_ids:String,
}
