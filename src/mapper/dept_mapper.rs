use rbatis::{crud,html_sql};
use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use crate::model::dept_model::DeptList;

#[html_sql("src/mapper/xml/dept_xml.html")]
pub async fn get_dept_list(rb: &mut dyn Executor,dept_name:Option<String>,status:Option<String>)->rbatis::Result<Vec<DeptList>>{
    impled!()
}