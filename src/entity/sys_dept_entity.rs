use rbatis::rbdc::datetime::DateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SysDeptEntity{
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
    pub create_time:Option<DateTime>,
    pub update_by:Option<String>,
    pub update_time:Option<DateTime>,
}