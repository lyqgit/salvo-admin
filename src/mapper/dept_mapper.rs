use rbatis::{crud,html_sql};
use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use crate::entity::sys_dept_entity::SysDeptEntity;
use crate::model::dept_model::DeptList;

#[html_sql("src/mapper/xml/dept_xml.html")]
pub async fn get_dept_list(rb: &mut dyn Executor,dept_name:Option<String>,status:Option<String>)->rbatis::Result<Vec<DeptList>>{
    impled!()
}

#[html_sql("src/mapper/xml/dept_xml.html")]
pub async fn get_dept_tree_by_id(rb: &mut dyn Executor,dept_id:i64)->rbatis::Result<Vec<DeptList>>{
    impled!()
}

#[html_sql("src/mapper/xml/dept_xml.html")]
pub async fn get_dept_by_id(rb: &mut dyn Executor,dept_id:i64)->rbatis::Result<Vec<DeptList>>{
    impled!()
}

#[html_sql("src/mapper/xml/dept_xml.html")]
pub async fn get_dept_list_exclude_id(rb: &mut dyn Executor,dept_id:i64)->rbatis::Result<Vec<DeptList>>{
    impled!()
}

#[html_sql("src/mapper/xml/dept_xml.html")]
pub async fn del_dept_by_id(rb: &mut dyn Executor,dept_id:String)->rbatis::Result<ExecResult>{
    impled!()
}

crud!(SysDeptEntity{},"sys_dept");