use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SysNoticeEntity{
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