use crate::mapper::dict_mapper;
use crate::GLOBAL_DB;
use crate::entity::sys_dict_type_entity::SysDictType;
use crate::model::common_model::Page;

pub async fn get_dict_by_page(page_num:u64,page_size:u64,dict_name:&str,dict_type:&str,status:&str,begin_time:&str,end_time:&str)->rbatis::Result<Page<SysDictType>>{
  let mut num = 0;
  if page_num >1{
    num = page_num - 1
  }
  let mut size = 10;
  if page_size >1{
    size = page_size
  }
  let list:Vec<SysDictType> = dict_mapper::select_dcit_type_by_page(&mut GLOBAL_DB.clone(),dict_name,dict_type,status,begin_time,end_time,num,size).await?;
  let count:u64 = dict_mapper::select_dcit_type_by_count(&mut GLOBAL_DB.clone(),dict_name,dict_type,status,begin_time,end_time).await?;
  Ok(Page { rows: list, total: count })
}
