use crate::entity::sys_dict_data_entity::{SysDictData,SysDictDataEntity};
use salvo::oapi::extract::JsonBody;
use crate::entity::sys_user_entity::SysUser;
use crate::mapper::dict_mapper;
use crate::GLOBAL_DB;
use crate::entity::sys_dict_type_entity::{SysDictType,ModifySysDictType};
use crate::model::common_model::Page;
use crate::utils::func;
use rbatis::rbdc::datetime::DateTime;
use crate::model::dict_model::{AddSysDictDataVo, EditSysDictData};

pub async fn get_dict_by_page(page_num:u64,page_size:u64,dict_name:&str,dict_type:&str,status:&str,begin_time:&str,end_time:&str)->rbatis::Result<Page<SysDictType>>{

  let (num,size) = func::create_page(page_num, page_size);
  let list:Vec<SysDictType> = dict_mapper::select_dict_type_by_page(&mut GLOBAL_DB.clone(),dict_name,dict_type,status,begin_time,end_time,num,size).await?;
  let count:u64 = dict_mapper::select_dict_type_by_count(&mut GLOBAL_DB.clone(),dict_name,dict_type,status,begin_time,end_time).await?;
  Ok(Page { rows: list, total: count })
}

pub async fn get_dict_data_by_type(dict_type:&str)->rbatis::Result<Vec<SysDictData>>{
  let list = dict_mapper::select_dict_data_by_type(&mut GLOBAL_DB.clone(),dict_type).await?;
  Ok(list)
}

pub async fn get_dict_by_id(dict_id:i64)->rbatis::Result<Option<SysDictType>>{
  let st = dict_mapper::select_dict_by_id(&mut GLOBAL_DB.clone(),dict_id).await?;
  Ok(st)
}

pub async fn add_dict_type(user_id:i32,dict_name:String,dict_type:String,status:String,remark:Option<String>)->rbatis::Result<bool>{
  let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
  let user = user.get(0).unwrap();
  let sys_dict_type = ModifySysDictType{dict_id:0,dict_name,dict_type,status,create_by:user.user_name.clone(),create_time:DateTime::now(),update_by:None,update_time:None,remark:remark};
  let rows = ModifySysDictType::insert(&mut GLOBAL_DB.clone(), &sys_dict_type).await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn edit_dict_type(user_id:i32,dict:JsonBody<ModifySysDictType>)->rbatis::Result<bool>{
  let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
  let user = user.get(0).unwrap();
  let sys_dict_type = ModifySysDictType{
    dict_id:dict.dict_id,
    dict_name:dict.dict_name.clone(),
    dict_type:dict.dict_type.clone(),
    status:dict.status.clone(),
    create_by:dict.create_by.clone(),
    create_time:dict.create_time.clone(),
    update_by:Some(user.user_name.clone()),
    update_time:Some(DateTime::now()),
    remark:dict.remark.clone()
  };
  let rows = ModifySysDictType::update_by_column(&mut GLOBAL_DB.clone(), &sys_dict_type,"dict_id").await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn del_dict_type(dict_id:Vec<&str>)->rbatis::Result<bool>{
  let rows = dict_mapper::del_dict_by_id(&mut GLOBAL_DB.clone(),dict_id).await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn get_all_dict_type()->rbatis::Result<Vec<SysDictType>>{
  let list = dict_mapper::select_all_dict_type(&mut GLOBAL_DB.clone()).await?;
  Ok(list)
}

pub async fn get_dict_data_by_page(page_num:u64,page_size:u64,dict_type:String,status:Option<String>,dict_label:Option<String>)->rbatis::Result<Page<SysDictData>>{
  let (num,size) = func::create_page(page_num, page_size);
  let list:Vec<SysDictData> = dict_mapper::select_dict_type_data_by_page(&mut GLOBAL_DB.clone(),dict_label.clone(),dict_type.clone(),status.clone(),num,size).await?;
  let count:u64 = dict_mapper::select_dict_type_data_by_count(&mut GLOBAL_DB.clone(),dict_label.clone(),dict_type.clone(),status.clone()).await?;
  Ok(Page { rows: list, total: count })
}

pub async fn add_dict_data(user_id:i32,payload:AddSysDictDataVo)->rbatis::Result<bool>{
  let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
  let user = user.get(0).unwrap();
  let mut sys_dict_data:SysDictDataEntity = payload.into();
  sys_dict_data.create_by = user.user_name.clone();
  let rows = SysDictDataEntity::insert(&mut GLOBAL_DB.clone(),&sys_dict_data).await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn del_dict_data_type(dict_code:Vec<&str>)->rbatis::Result<bool>{
  let rows = dict_mapper::del_dict_data_by_id(&mut GLOBAL_DB.clone(),dict_code).await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn select_dict_data_by_id(dict_code:i64)->rbatis::Result<Option<SysDictData>>{
  let list = dict_mapper::select_dict_data_by_id(&mut GLOBAL_DB.clone(),dict_code).await?;
  let one = list.get(0).cloned();
  Ok(one)
}

pub async fn edit_dict_data(payload:EditSysDictData)->rbatis::Result<bool>{
  let sys_dict_data_entity:SysDictDataEntity = payload.into();
  let rows = SysDictDataEntity::update_by_column(&mut GLOBAL_DB.clone(), &sys_dict_data_entity, "dict_code").await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}