use rbatis::executor::Executor;
use rbatis::{crud, html_sql};
use crate::entity::sys_post_entity::SysPostEntity;
use crate::model::post_model::SysPostList;

#[html_sql("src/mapper/xml/post_xml.html")]
pub async fn get_post_page(rb: &mut dyn Executor,page_num:u64,page_size:u64,post_code:Option<String>,post_name:Option<String>,status:Option<String>)->rbatis::Result<Vec<SysPostList>>{
    impled!()
}

#[html_sql("src/mapper/xml/post_xml.html")]
pub async fn get_post_count(rb: &mut dyn Executor,post_code:Option<String>,post_name:Option<String>,status:Option<String>)->rbatis::Result<u64>{
    impled!()
}

#[html_sql("src/mapper/xml/post_xml.html")]
pub async fn get_post_by_id(rb: &mut dyn Executor,post_id:i64)->rbatis::Result<Vec<SysPostList>>{
    impled!()
}

crud!(SysPostEntity{},"sys_post");