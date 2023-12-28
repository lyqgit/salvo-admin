use rbatis::{crud,html_sql};
use rbatis::executor::Executor;
use crate::entity::excel_entity::ToolExcelEntity;
use crate::model::excel_model::{ExcelList,ExcelDetail};

#[html_sql("src/mapper/xml/excel_xml.html")]
pub async fn get_excel_list_by_id(rb: &mut dyn Executor,user_id:i32)->rbatis::Result<Vec<ExcelList>>{
    impled!()
}

#[html_sql("src/mapper/xml/excel_xml.html")]
pub async fn get_excel_list(rb: &mut dyn Executor)->rbatis::Result<Vec<ExcelList>>{
    impled!()
}

#[html_sql("src/mapper/xml/excel_xml.html")]
pub async fn get_excel_detail_by_id(rb: &mut dyn Executor,excel_id:String)->rbatis::Result<Vec<ExcelDetail>>{
    impled!()
}

crud!(ToolExcelEntity{},"tool_excel");