use rbatis::{crud,html_sql};
use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use crate::entity::sys_role_menu_entity::SysRoleMenuEntity;

#[html_sql("src/mapper/xml/role_menu_xml.html")]
pub async fn del_role_menu_by_role_id(rb: &mut dyn Executor,role_id:String)->rbatis::Result<ExecResult>{
    impled!()
}

#[html_sql("src/mapper/xml/role_menu_xml.html")]
pub async fn get_menu_id_by_role_id(rb: &mut dyn Executor,role_id:i32)->rbatis::Result<Vec<SysRoleMenuEntity>>{
    impled!()
}

crud!(SysRoleMenuEntity{},"sys_role_menu");