use rbatis::{html_sql, executor::Executor};
use crate::entity::sys_role_entity::SysRole;

#[html_sql("src/mapper/xml/role_xml.html")]
pub async fn select_roles_by_user_id(rb: &mut dyn Executor,id:i32)->rbatis::Result<Vec<SysRole>>{
  impled!()
}