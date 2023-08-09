use rbatis::{crud,html_sql};
use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use crate::entity::sys_user_role_entity::SysUserRoleEntity;

crud!(SysUserRoleEntity{},"sys_user_role");

#[html_sql("src/mapper/xml/user_role_xml.html")]
pub async fn select_role_id_by_user_id(rb: &mut dyn Executor,user_id:i64)->rbatis::Result<Vec<SysUserRoleEntity>>{
    impled!()
}

#[html_sql("src/mapper/xml/user_role_xml.html")]
pub async fn del_by_role_and_user_id(rb: &mut dyn Executor,user_id:i64,role_id:String)->rbatis::Result<ExecResult>{
    impled!()
}

#[html_sql("src/mapper/xml/user_role_xml.html")]
pub async fn del_by_role_and_user_id_more(rb: &mut dyn Executor,user_id:String,role_id:i64)->rbatis::Result<ExecResult>{
    impled!()
}