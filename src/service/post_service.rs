use rbatis::rbdc::datetime::DateTime;
use crate::entity::sys_post_entity::SysPostEntity;
use crate::entity::sys_user_entity::SysUser;
use crate::GLOBAL_DB;
use crate::mapper::post_mapper;
use crate::model::common_model::Page;
use crate::model::post_model::SysPostList;
use crate::utils::func::{create_page, create_page_list, is_modify_ok};

pub async fn get_post_page(page_num:u64,page_size:u64,post_code:Option<String>,post_name:Option<String>,status:Option<String>)->rbatis::Result<Page<SysPostList>>{
    let (num,size) = create_page(page_num,page_size);
    let list = post_mapper::get_post_page(&mut GLOBAL_DB.clone(),num,size,post_code.clone(),post_name.clone(),status.clone()).await?;
    let total = post_mapper::get_post_count(&mut GLOBAL_DB.clone(),post_code,post_name,status).await?;
    Ok(create_page_list(list,total))
}

pub async fn post_add_post(user_id:i32,post_code:Option<String>,post_name:Option<String>,post_sort:Option<i8>,status:Option<String>,remark:Option<String>)->rbatis::Result<bool>{
    let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
    let user = user.get(0).unwrap();
    let sys_post = SysPostEntity{
        post_id: None,
        post_code,
        post_name,
        post_sort,
        status,
        create_by: Some(user.user_name.clone()),
        create_time: Some(DateTime::now()),
        update_by: None,
        update_time: None,
        remark
    };
    let rows = SysPostEntity::insert(&mut GLOBAL_DB.clone(),&sys_post).await?;
    Ok(is_modify_ok(rows.rows_affected))
}

pub async fn get_post_by_id(id:i64)->rbatis::Result<Option<SysPostList>>{
    let list = post_mapper::get_post_by_id(&mut GLOBAL_DB.clone(),id).await?;
    let one = list.get(0).cloned();
    Ok(one)
}


pub async fn post_edit_post(user_id:i32,post_id:Option<i64>,post_code:Option<String>,post_name:Option<String>,post_sort:Option<i8>,status:Option<String>,remark:Option<String>)->rbatis::Result<bool>{
    let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
    let user = user.get(0).unwrap();
    let sys_post = SysPostEntity{
        post_id,
        post_code,
        post_name,
        post_sort,
        status,
        create_by: None,
        create_time: None,
        update_by: Some(user.user_name.clone()),
        update_time: Some(DateTime::now()),
        remark
    };
    let rows = SysPostEntity::update_by_column(&mut GLOBAL_DB.clone(),&sys_post,"post_id").await?;
    Ok(is_modify_ok(rows.rows_affected))
}

pub async fn del_post_by_id(post_id:String)->rbatis::Result<bool>{
    let rows = post_mapper::del_post_by_id(&mut GLOBAL_DB.clone(),post_id).await?;
    Ok(is_modify_ok(rows.rows_affected))
}

