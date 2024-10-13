use rbatis::executor::Executor;
use rbatis::{crud, html_sql};
use rbatis::rbdc::db::ExecResult;
use crate::entity::sys_notice_entity::SysNoticeEntity;
use crate::model::notice_model::SysNoticeList;

#[html_sql("src/mapper/xml/notice_xml.html")]
pub async fn get_notice_page(rb: &mut dyn Executor,page_num:u64,page_size:u64,notice_title:Option<String>,notice_type:Option<String>,status:Option<String>)->rbatis::Result<Vec<SysNoticeList>>{
    impled!()
}

#[html_sql("src/mapper/xml/notice_xml.html")]
pub async fn get_notice_count(rb: &mut dyn Executor,notice_title:Option<String>,notice_type:Option<String>,status:Option<String>)->rbatis::Result<u64>{
    impled!()
}

#[html_sql("src/mapper/xml/notice_xml.html")]
pub async fn get_notice_by_id(rb: &mut dyn Executor,notice_id:i64)->rbatis::Result<Vec<SysNoticeList>>{
    impled!()
}

#[html_sql("src/mapper/xml/notice_xml.html")]
pub async fn del_notice_by_id(rb: &mut dyn Executor,notice_id:String)->rbatis::Result<ExecResult>{
    impled!()
}

#[html_sql("src/mapper/xml/notice_xml.html")]
pub async fn get_notice_list(rb: &mut dyn Executor)->rbatis::Result<Vec<SysNoticeList>>{
    impled!()
}

#[html_sql("src/mapper/xml/notice_xml.html")]
pub async fn get_notice_list_by_user_id(rb: &mut dyn Executor,user_id:i32)->rbatis::Result<Vec<SysNoticeList>>{
    impled!()
}


crud!(SysNoticeEntity{},"sys_notice");