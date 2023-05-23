use salvo::oapi::extract::{JsonBody, QueryParam};
use salvo::Depot;
use salvo::{oapi::endpoint, Request};
use crate::entity::sys_dict_data_entity::SysDictData;
use crate::model::common_model::Page;
use crate::model::dict_model::{AddDictType, DictTypeDataPagePayload, AddSysDictDataVo, DictTypePagePayload, DictDataDetail, EditSysDictData};
use crate::utils::res::{res_json_ok,res_json_custom,ResObj};
use crate::{service::dict_service, utils::res::Res, entity::sys_dict_type_entity::{SysDictType,ModifySysDictType}};


#[endpoint(
  parameters(DictTypePagePayload),
  responses(
    (status = 200,body=ResObj<Page<SysDictType>>,description ="字典类型列表")
  ),
)]
pub async fn get_dict_list(req:&mut Request)->Res<Page<SysDictType>>{
  let page_num = req.query::<u64>("pageNum").map_or(1, |v|v);
  let page_size = req.query::<u64>("pageSize").map_or(10, |v|v);
  let dict_name = req.query::<&str>("dictName").map_or("", |v|v);
  let dict_type = req.query::<&str>("dictType").map_or("", |v|v);
  let status = req.query::<&str>("status").map_or("", |v|v);
  let begin_time = req.query::<&str>("params[beginTime]").map_or("", |v|v);
  let end_time = req.query::<&str>("params[endTime]").map_or("", |v|v);

  match dict_service::get_dict_by_page(page_num,page_size,dict_name,dict_type,status,begin_time,end_time).await{
    Ok(v)=>{
      Ok(res_json_ok(Some(v)))
    },
    Err(err)=>{
      Err(res_json_custom(400,err.to_string()))
    }
  }
}

#[endpoint(
  responses(
    (status = 200,body=ResObj<Vec<SysDictData>>,description ="根据类型获取字典数据列表")
  ),
)]
pub async fn get_dict_list_by_type(req:&mut Request)->Res<Vec<SysDictData>>{
  let code = req.param::<&str>("type").map_or("", |v|v);
  match dict_service::get_dict_data_by_type(code).await {
    Ok(v)=>{
      Ok(res_json_ok(Some(v)))
    },
    Err(err)=>{
      Err(res_json_custom(400,err.to_string()))
    }
  }
}

#[endpoint(
  responses(
    (status = 200,body=ResObj<Option<SysDictType>>,description ="根据id获取字典类型数据")
  ),
)]
pub async fn get_dict_by_id(req:&mut Request)->Res<Option<SysDictType>>{
  let code = req.param::<i64>("id").map_or(0, |v|v);
  println!("获取参数{}",code);
  match dict_service::get_dict_by_id(code).await {
    Ok(v)=>{
      Ok(res_json_ok(Some(v)))
    },
    Err(err)=>{
      Err(res_json_custom(400,err.to_string()))
    }
  }
}

#[endpoint(
  responses(
    (status = 200,body=ResObj<Vec<SysDictType>>,description ="获取所有字典类型")
  ),
)]
pub async fn get_all_dict_type()->Res<Vec<SysDictType>>{
  match dict_service::get_all_dict_type().await {
    Ok(v)=>{
      Ok(res_json_ok(Some(v)))
    },
    Err(err)=>{
      Err(res_json_custom(400,err.to_string()))
    }
  }
}


#[endpoint(
  responses(
    (status = 200,body=ResObj<()>,description ="创建字典")
  )
)]
pub async fn add_dict_type(dict:JsonBody<AddDictType>,depot:&mut Depot)->Res<()>{
  let user_id = depot.get::<i32>("userId").copied().unwrap();
  match dict_service::add_dict_type(
    user_id,
    dict.dict_name.clone(),
    dict.dict_type.clone(),
    dict.status.clone(),
    dict.remark.clone(),
  ).await {
    Ok(v)=>{
      if v {
        Ok(res_json_ok(Some(())))
      }else{
        Err(res_json_custom(400,"添加失败".to_string()))
      }
    },
    Err(err)=>{
      Err(res_json_custom(400,err.to_string()))
    }
  }
}


#[endpoint(
  responses(
    (status = 200,body=ResObj<()>,description ="创建字典")
  )
)]
pub async fn edit_dict_type(dict:JsonBody<ModifySysDictType>,depot:&mut Depot)->Res<()>{
  let user_id = depot.get::<i32>("userId").copied().unwrap();
  match dict_service::edit_dict_type(
    user_id,
    dict
  ).await {
    Ok(v)=>{
      if v {
        Ok(res_json_ok(Some(())))
      }else{
        Err(res_json_custom(400,"修改失败".to_string()))
      }
    },
    Err(err)=>{
      Err(res_json_custom(400,err.to_string()))
    }
  }
}

#[endpoint(
  responses(
    (status = 200,body=ResObj<()>,description ="删除字典类型")
  )
)]
pub async fn del_dict_type(req:&mut Request)->Res<()>{
  let code = req.param::<&str>("id").map_or("", |v|v);
  let code = code.split(",").collect::<Vec<&str>>();
  if code.len() == 0{
    return Err(res_json_custom(400,"缺少参数".to_string()));
  }
  match dict_service::del_dict_type(code).await {
    Ok(v)=>{
      if v {
        Ok(res_json_ok(Some(())))
      }else{
        Err(res_json_custom(400,"删除失败".to_string()))
      }
    },
    Err(err)=>{
      Err(res_json_custom(400,err.to_string()))
    }
  }
}

#[endpoint(
  parameters(
    DictTypeDataPagePayload
  ),
  responses(
    (status = 200,body=ResObj<Page<SysDictData>>,description ="字典类型数据列表")
  ),
)]
pub async fn get_dict_data_list(req:&mut Request)->Res<Page<SysDictData>>{
  let payload:DictTypeDataPagePayload = req.parse_queries().unwrap();
  match dict_service::get_dict_data_by_page(payload.page_num,payload.page_size,payload.dict_type.clone(),payload.status.clone(),payload.dict_label.clone()).await{
    Ok(v)=>{
      Ok(res_json_ok(Some(v)))
    },
    Err(err)=>{
      Err(res_json_custom(400,err.to_string()))
    }
  }
}

#[endpoint(
  responses(
    (status = 200,body=ResObj<()>,description ="创建字典数据")
  )
)]
pub async fn post_add_dict_data(payload:JsonBody<AddSysDictDataVo>,depot:&mut Depot)->Res<()>{
  let user_id = depot.get::<i32>("userId").copied().unwrap();
  match dict_service::add_dict_data(user_id,payload.into_inner()).await{
    Ok(v)=>{
      if v {
        Ok(res_json_ok(Some(())))
      }else{
        Err(res_json_custom(400,"删除失败".to_string()))
      }
    },
    Err(err)=>{
      Err(res_json_custom(400,err.to_string()))
    }
  }
}


#[endpoint(
  responses(
    (status = 200,body=ResObj<()>,description ="删除字典类型数据")
  )
)]
pub async fn del_dict_type_data(req:&mut Request)->Res<()>{
  let code = req.param::<&str>("id").map_or("", |v|v);
  let code = code.split(",").collect::<Vec<&str>>();
  if code.len() == 0{
    return Err(res_json_custom(400,"缺少参数".to_string()));
  }
  match dict_service::del_dict_data_type(code).await {
    Ok(v)=>{
      if v {
        Ok(res_json_ok(Some(())))
      }else{
        Err(res_json_custom(400,"删除失败".to_string()))
      }
    },
    Err(err)=>{
      Err(res_json_custom(400,err.to_string()))
    }
  }
}


#[endpoint(
  parameters(
    DictDataDetail
  ),
  responses(
    (status = 200,body=ResObj<Option<SysDictData>>,description ="字典类型数据详情")
  )
)]
pub async fn get_dict_type_data_by_id(req:&mut Request)->Res<Option<SysDictData>>{
  let code:DictDataDetail = req.parse_params().unwrap();
  match dict_service::select_dict_data_by_id(code.id).await {
    Ok(v)=>{
      Ok(res_json_ok(Some(v)))
    },
    Err(err)=>{
      Err(res_json_custom(400,err.to_string()))
    }
  }
}


#[endpoint(
  responses(
    (status = 200,body=ResObj<()>,description ="修改字典数据")
  )
)]
pub async fn put_edit_dict_data(payload:JsonBody<EditSysDictData>)->Res<()>{
  match dict_service::edit_dict_data(payload.into_inner()).await{
    Ok(v)=>{
      if v {
        Ok(res_json_ok(Some(())))
      }else{
        Err(res_json_custom(400,"删除失败".to_string()))
      }
    },
    Err(err)=>{
      Err(res_json_custom(400,err.to_string()))
    }
  }
}