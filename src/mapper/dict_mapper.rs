use rbatis::{executor::Executor, html_sql, crud};
use crate::entity::{sys_dict_type_entity::{SysDictType,ModifySysDictType}, sys_dict_data_entity::{SysDictData,SysDictDataEntity}};
use rbatis::rbdc::db::ExecResult;

#[html_sql("src/mapper/xml/dict_xml.html")]
pub async fn select_dict_type_by_page(rb: &mut dyn Executor,dict_name:&str,dict_type:&str,status:&str,begin_time:&str,end_time:&str,page_num:u64,page_size:u64)->rbatis::Result<Vec<SysDictType>>{
  impled!()
}

#[html_sql("src/mapper/xml/dict_xml.html")]
pub async fn select_dict_type_by_count(rb: &mut dyn Executor,dict_name:&str,dict_type:&str,status:&str,begin_time:&str,end_time:&str)->rbatis::Result<u64>{
  impled!()
}

#[html_sql("src/mapper/xml/dict_xml.html")]
pub async fn select_dict_data_by_type(rb: &mut dyn Executor,dict_type:&str)->rbatis::Result<Vec<SysDictData>>{
  impled!()
}


#[html_sql("src/mapper/xml/dict_xml.html")]
pub async fn select_dict_by_id(rb: &mut dyn Executor,dict_id:i64)->rbatis::Result<Option<SysDictType>>{
  impled!()
}

#[html_sql("src/mapper/xml/dict_xml.html")]
pub async fn del_dict_by_id(rb: &mut dyn Executor,arr:Vec<&str>)->rbatis::Result<ExecResult>{
  impled!()
}

#[html_sql("src/mapper/xml/dict_xml.html")]
pub async fn select_all_dict_type(rb: &mut dyn Executor)->rbatis::Result<Vec<SysDictType>>{
  impled!()
}


#[html_sql("src/mapper/xml/dict_xml.html")]
pub async fn select_dict_type_data_by_page(rb: &mut dyn Executor,dict_label:Option<String>,dict_type:String,status:Option<String>,page_num:u64,page_size:u64)->rbatis::Result<Vec<SysDictData>>{
  impled!()
}

#[html_sql("src/mapper/xml/dict_xml.html")]
pub async fn select_dict_type_data_by_count(rb: &mut dyn Executor,dict_label:Option<String>,dict_type:String,status:Option<String>)->rbatis::Result<u64>{
  impled!()
}

#[html_sql("src/mapper/xml/dict_xml.html")]
pub async fn del_dict_data_by_id(rb: &mut dyn Executor,arr:Vec<&str>)->rbatis::Result<ExecResult>{
  impled!()
}

#[html_sql("src/mapper/xml/dict_xml.html")]
pub async fn select_dict_data_by_id(rb: &mut dyn Executor,dict_code:i64)->rbatis::Result<Vec<SysDictData>>{
  impled!()
}

crud!(SysDictType{});
crud!(SysDictDataEntity{},"sys_dict_data");
crud!(ModifySysDictType{},"sys_dict_type");