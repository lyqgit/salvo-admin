use crate::GLOBAL_DB;
use crate::mapper::dept_mapper;
use crate::model::dept_model::DeptList;

pub async fn get_dept_list(dept_name:Option<String>,status:Option<String>) ->rbatis::Result<Vec<DeptList>>{
    let list = dept_mapper::get_dept_list(&mut GLOBAL_DB.clone(),dept_name,status).await?;
    Ok(list)
}