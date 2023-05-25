use rbatis::{html_sql, executor::Executor,crud};
use crate::entity::sys_menu_entity::SysMenu;
use crate::model::menu_model::SysMenuPage;

#[html_sql("src/mapper/xml/menu_xml.html")]
pub async fn select_menus_by_role_id(rb: &mut dyn Executor,is_admin:bool,id:String)->rbatis::Result<Vec<SysMenu>>{
  impled!()
}

#[html_sql("src/mapper/xml/menu_xml.html")]
pub async fn select_menus_by_user_id(rb: &mut dyn Executor,is_admin:bool,id:i32)->rbatis::Result<Vec<SysMenu>>{
  impled!()
}

#[html_sql("src/mapper/xml/menu_xml.html")]
pub async fn select_menus_list(rb: &mut dyn Executor,menu_name:Option<String>,status:Option<String>)->rbatis::Result<Vec<SysMenuPage>>{
  impled!()
}

#[html_sql("src/mapper/xml/menu_xml.html")]
pub async fn select_menus_by_id(rb: &mut dyn Executor,menu_id:i64)->rbatis::Result<Vec<SysMenuPage>>{
  impled!()
}

crud!(SysMenu{});
