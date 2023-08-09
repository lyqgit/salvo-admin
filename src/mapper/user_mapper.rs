use crate::entity::sys_user_entity::{SysUser, SysUserEntity,AddSysUserEntity};
use rbatis::{crud, html_sql, impl_select};
use rbatis::executor::Executor;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use crate::model::user_model::SysUserList;

crud!(SysUser{},"sys_user");

impl_select!(SysUser{select_user_by_up(username:String,password:String)=>"`where user_name = #{username} and password = #{password} limit 1`"});

#[html_sql("src/mapper/xml/user_xml.html")]
pub async fn get_user_page(rb: &mut dyn Executor,page_num:u64,page_size:u64,user_name:Option<String>,phone_number:Option<String>,status:Option<String>,begin_time:Option<DateTime>,end_time:Option<DateTime>,dept_id:Option<i64>)->rbatis::Result<Vec<SysUserList>>{
    impled!()
}

#[html_sql("src/mapper/xml/user_xml.html")]
pub async fn get_user_count(rb: &mut dyn Executor,user_name:Option<String>,phone_number:Option<String>,status:Option<String>,begin_time:Option<DateTime>,end_time:Option<DateTime>,dept_id:Option<i64>)->rbatis::Result<u64>{
    impled!()
}

#[html_sql("src/mapper/xml/user_xml.html")]
pub async fn get_user_by_id(rb: &mut dyn Executor,user_id:Option<i64>)->rbatis::Result<Vec<SysUserList>>{
    impled!()
}

#[html_sql("src/mapper/xml/user_xml.html")]
pub async fn update_user_status_by_id(rb: &mut dyn Executor,status:String,user_id:i64)->rbatis::Result<ExecResult>{
    impled!()
}

crud!(SysUserEntity{},"sys_user");
crud!(AddSysUserEntity{},"sys_user");
