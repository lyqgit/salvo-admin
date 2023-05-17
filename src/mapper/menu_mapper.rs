use rbatis::{html_sql, executor::Executor};

#[html_sql("src/mapper/xml/menu_xml.html")]
async fn select_menus_by_role_id(rb: &mut dyn Executor,is_admin:bool,id:String)->rbatis::Result<Vec<String>>{
  impled!()
}