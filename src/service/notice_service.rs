use rbatis::rbdc::datetime::DateTime;
use crate::entity::sys_notice_entity::SysNoticeEntity;
use crate::entity::sys_user_entity::SysUser;
use crate::GLOBAL_DB;
use crate::mapper::notice_mapper;
use crate::model::common_model::Page;
use crate::model::notice_model::SysNoticeList;
use crate::utils::func::{create_page, create_page_list, is_modify_ok};

pub async fn get_notice_page(page_num:u64,page_size:u64,notice_title:Option<String>,notice_type:Option<String>,status:Option<String>)->rbatis::Result<Page<SysNoticeList>>{
    let (num,size) = create_page(page_num,page_size);
    let list = notice_mapper::get_notice_page(&mut GLOBAL_DB.clone(),num,size,notice_title.clone(),notice_type.clone(),status.clone()).await?;
    let total = notice_mapper::get_notice_count(&mut GLOBAL_DB.clone(),notice_title,notice_type,status).await?;
    Ok(create_page_list(list,total))
}

pub async fn notice_add_notice(user_id:i32,notice_title:Option<String>,notice_type:Option<String>,notice_content:Option<String>,status:Option<String>,remark:Option<String>)->rbatis::Result<bool>{
    let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
    let user = user.get(0).unwrap();
    let sys_notice = SysNoticeEntity{
        notice_id: None,
        notice_title,
        notice_type,
        notice_content,
        status,
        create_by: Some(user.user_name.clone()),
        create_time: Some(DateTime::now()),
        update_by: None,
        update_time: None,
        remark
    };
    let rows = SysNoticeEntity::insert(&mut GLOBAL_DB.clone(),&sys_notice).await?;
    Ok(is_modify_ok(rows.rows_affected))
}

pub async fn get_notice_by_id(id:i64)->rbatis::Result<Option<SysNoticeList>>{
    let list = notice_mapper::get_notice_by_id(&mut GLOBAL_DB.clone(),id).await?;
    let one = list.get(0).cloned();
    Ok(one)
}


pub async fn notice_edit_notice(user_id:i32,notice_id:Option<i64>,notice_title:Option<String>,notice_type:Option<String>,notice_content:Option<String>,status:Option<String>,remark:Option<String>)->rbatis::Result<bool>{
    let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
    let user = user.get(0).unwrap();
    let sys_notice = SysNoticeEntity{
        notice_id,
        notice_title,
        notice_type,
        notice_content,
        status,
        create_by: None,
        create_time: None,
        update_by: Some(user.user_name.clone()),
        update_time: Some(DateTime::now()),
        remark
    };
    let rows = SysNoticeEntity::update_by_column(&mut GLOBAL_DB.clone(),&sys_notice,"notice_id").await?;
    Ok(is_modify_ok(rows.rows_affected))
}

pub async fn del_notice_by_id(notice_id:String)->rbatis::Result<bool>{
    let rows = notice_mapper::del_notice_by_id(&mut GLOBAL_DB.clone(),notice_id).await?;
    Ok(is_modify_ok(rows.rows_affected))
}

