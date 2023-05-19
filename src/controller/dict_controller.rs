use salvo::{oapi::endpoint, Request};
use crate::model::common_model::Page;
use crate::utils::res::{res_json_ok,res_json_custom,ResObj};
use crate::{service::dict_service, utils::res::Res, entity::sys_dict_type_entity::SysDictType};


#[endpoint(
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