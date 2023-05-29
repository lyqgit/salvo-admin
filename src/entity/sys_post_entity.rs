use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SysPostEntity{
    pub post_id:Option<i64>,
    pub post_code:Option<String>,
    pub post_name:Option<String>,
    pub post_sort:Option<i8>,
    pub status:Option<String>,
    pub create_by:Option<String>,
    pub create_time:Option<DateTime>,
    pub update_by:Option<String>,
    pub update_time:Option<DateTime>,
    pub remark:Option<String>,
}