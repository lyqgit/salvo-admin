use salvo::{oapi::{ToSchema}};
use salvo::oapi::ToParameters;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,ToSchema,Deserialize,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct DeptList{
    pub dept_id:Option<i64>,
    pub parent_id:Option<i64>,
    pub ancestors:Option<String>,
    pub dept_name:Option<String>,
    pub order_num:Option<i8>,
    pub leader:Option<String>,
    pub phone:Option<String>,
    pub email:Option<String>,
    pub status:Option<String>,
    pub del_flag:Option<String>,
    pub create_by:Option<String>,
    pub create_time:Option<String>,
    pub update_by:Option<String>,
    pub update_time:Option<String>,
}

#[derive(Debug,Serialize,ToParameters,Deserialize,Clone)]
#[salvo(parameters(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
#[salvo(parameters(parameter_in = Query))]
pub struct DeptListPayload{
    pub dept_name:Option<String>,
    pub status:Option<String>,
}

#[derive(Debug,Serialize,ToSchema,Deserialize,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
pub struct DeptModifyPayload{
    pub dept_id:Option<i64>,
    pub parent_id:i64,
    pub dept_name:String,
    pub order_num:i8,
    pub leader:Option<String>,
    pub phone:Option<String>,
    pub email:Option<String>,
    pub status:Option<String>,
}

#[derive(Debug,Serialize,ToSchema,Clone)]
pub struct DeptTree{
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(value_type=Option<Vec<Object>>))]
    pub children:Option<Vec<DeptTree>>,
    pub id:i64,
    pub label:String,
}
