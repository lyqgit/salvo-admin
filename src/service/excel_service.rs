use rbatis::rbdc::datetime::DateTime;
use uuid::Uuid;
use crate::GLOBAL_DB;
use crate::entity::excel_entity::ToolExcelEntity;
use crate::utils::func::is_modify_ok;
use crate::model::excel_model::{ExcelList,ExcelDetail};
use crate::mapper::excel_mapper;

pub async fn add_excel(excel_name:Option<String>,excel_data:Option<String>,user_id:i32)->rbatis::Result<String>{
    let uuid = Uuid::new_v4().to_string();
    let tool_excel_entity = ToolExcelEntity{
        excel_id: Some(uuid.clone()),
        excel_name:excel_name,
        excel_data,
        user_id:Some(user_id),
        create_time: Some(DateTime::now()),
        update_time: None,
    };
    let rows = ToolExcelEntity::insert(&mut GLOBAL_DB.clone(),&tool_excel_entity).await?;
    if is_modify_ok(rows.rows_affected) {
        Ok(uuid)
    }else {
        Err(rbatis::Error::from(String::from("创建id错误")))
    }
}

pub async fn edit_excel(excel_id:Option<String>,excel_data:Option<String>)->rbatis::Result<bool>{
    let tool_excel_entity = ToolExcelEntity{
        excel_id,
        excel_name:None,
        excel_data,
        user_id:None,
        create_time: None,
        update_time: Some(DateTime::now()),
    };
    let rows = ToolExcelEntity::update_by_column(&mut GLOBAL_DB.clone(),&tool_excel_entity,"excel_id").await?;
    Ok(is_modify_ok(rows.rows_affected))
}

pub async fn excel_list_by_user_id(user_id:i32)->rbatis::Result<Vec<ExcelList>>{
    let list = excel_mapper::get_excel_list_by_id(&mut GLOBAL_DB.clone(),user_id).await?;
    Ok(list)
}

pub async fn excel_list()->rbatis::Result<Vec<ExcelList>>{
    let list = excel_mapper::get_excel_list(&mut GLOBAL_DB.clone()).await?;
    Ok(list)
}

pub async fn excel_detail(excel_id:String)->rbatis::Result<Option<ExcelDetail>>{
    let list = excel_mapper::get_excel_detail_by_id(&mut GLOBAL_DB.clone(),excel_id).await?;
    let one = list.get(0).cloned();
    Ok(one)
}