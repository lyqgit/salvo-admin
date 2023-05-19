use rbatis::{executor::Executor,html_sql};
use crate::entity::sys_dict_type_entity::SysDictType;

#[html_sql("src/mapper/xml/dict_xml.html")]
pub async fn select_dcit_type_by_page(rb: &mut dyn Executor,dict_name:&str,dict_type:&str,status:&str,begin_time:&str,end_time:&str,page_num:u64,page_size:u64)->rbatis::Result<Vec<SysDictType>>{
  impled!()
}

#[html_sql("src/mapper/xml/dict_xml.html")]
pub async fn select_dcit_type_by_count(rb: &mut dyn Executor,dict_name:&str,dict_type:&str,status:&str,begin_time:&str,end_time:&str)->rbatis::Result<u64>{
  impled!()
}
