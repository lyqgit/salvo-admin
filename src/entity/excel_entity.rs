use rbatis::rbdc::datetime::DateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct ToolExcelEntity{
    pub excel_id:Option<String>,
    pub excel_name:Option<String>,
    pub excel_data:Option<String>,
    pub user_id:Option<i32>,
    pub create_time:Option<DateTime>,
    pub update_time:Option<DateTime>,
}