use rbatis::rbdc::datetime::DateTime;
use crate::entity::sys_dept_entity::SysDeptEntity;
use crate::entity::sys_user_entity::SysUser;
use crate::GLOBAL_DB;
use crate::mapper::dept_mapper;
use crate::model::dept_model::{DeptList, DeptTree};
use crate::utils::func;
use crate::utils::func::is_modify_ok;

pub async fn get_dept_list(dept_name:Option<String>,status:Option<String>) ->rbatis::Result<Vec<DeptList>>{
    let list = dept_mapper::get_dept_list(&mut GLOBAL_DB.clone(),dept_name,status).await?;
    Ok(list)
}

pub async fn get_dept_list_exclude_id(dept_id:i64) ->rbatis::Result<Vec<DeptList>>{
    let list = dept_mapper::get_dept_list_exclude_id(&mut GLOBAL_DB.clone(),dept_id).await?;
    Ok(list)
}

pub async fn get_dept_by_id(dept_id:i64) ->rbatis::Result<Option<DeptList>>{
    let list = dept_mapper::get_dept_by_id(&mut GLOBAL_DB.clone(),dept_id).await?;
    let one = list.get(0).cloned();
    Ok(one)
}

pub async fn add_dept(user_id:i32,dept_name:String,order_num:i8,parent_id:i64,leader:Option<String>,email:Option<String>,phone:Option<String>,status:Option<String>)->rbatis::Result<bool>{
    let tree = dept_mapper::get_dept_tree_by_id(&mut GLOBAL_DB.clone(),parent_id).await?;
    let mut ancestors = Vec::<String>::new();
    ancestors.push("0".to_string());
    for (_,it) in tree.iter().enumerate(){
        ancestors.push(it.dept_id.unwrap().to_string())
    }

    let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
    let user = user.get(0).unwrap();
    let sys_dept = SysDeptEntity{
        dept_id: None,
        parent_id: Some(parent_id),
        ancestors: Some(ancestors.join(",").to_string()),
        dept_name: Some(dept_name),
        order_num: Some(order_num),
        leader,
        phone,
        email,
        status,
        del_flag: Some("0".to_string()),
        create_by: Some(user.user_name.clone()),
        create_time: Some(DateTime::now()),
        update_by: None,
        update_time: None,
    };
    let rows = SysDeptEntity::insert(&mut GLOBAL_DB.clone(),&sys_dept).await?;
    Ok(is_modify_ok(rows.rows_affected))
}


pub async fn edit_dept(user_id:i32,dept_id:Option<i64>,dept_name:String,order_num:i8,parent_id:i64,leader:Option<String>,email:Option<String>,phone:Option<String>,status:Option<String>)->rbatis::Result<bool>{
    let tree = dept_mapper::get_dept_tree_by_id(&mut GLOBAL_DB.clone(),parent_id).await?;
    let mut ancestors = Vec::<String>::new();
    ancestors.push("0".to_string());
    for (_,it) in tree.iter().enumerate(){
        ancestors.push(it.dept_id.unwrap().to_string())
    }

    let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
    let user = user.get(0).unwrap();
    let sys_dept = SysDeptEntity{
        dept_id,
        parent_id: Some(parent_id),
        ancestors: Some(ancestors.join(",").to_string()),
        dept_name: Some(dept_name),
        order_num: Some(order_num),
        leader,
        phone,
        email,
        status,
        del_flag: Some("0".to_string()),
        create_by: None,
        create_time: None,
        update_by: Some(user.user_name.clone()),
        update_time: Some(DateTime::now()),
    };
    let rows = SysDeptEntity::update_by_column(&mut GLOBAL_DB.clone(),&sys_dept,"dept_id").await?;
    Ok(is_modify_ok(rows.rows_affected))
}

pub async fn del_dept_by_id(dept_id:String)->rbatis::Result<bool>{
    let rows = dept_mapper::del_dept_by_id(&mut GLOBAL_DB.clone(),dept_id).await?;
    Ok(is_modify_ok(rows.rows_affected))
}

pub async fn get_dept_tree()->rbatis::Result<Vec<DeptTree>>{
    let list = dept_mapper::get_dept_list(&mut GLOBAL_DB.clone(),None,None).await?;
    let mut tree = Vec::<DeptTree>::new();
    func::dept_arr_to_tree(&mut tree,list,0);
    Ok(tree)
}