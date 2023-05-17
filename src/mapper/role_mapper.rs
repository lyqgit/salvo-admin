use rbatis::{html_sql, executor::Executor};

#[html_sql("src/mapper/xml/role_xml.html")]
async fn select_roles_by_user_id(rb: &mut dyn Executor,id:i32)->rbatis::Result<Vec<String>>{
  impled!()
}