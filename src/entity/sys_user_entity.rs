use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SysUserEntity{
  pub user_id:i64,
  pub dept_id:Option<i64>,
  pub user_name:String,
  pub nick_name:String,
  pub user_type:Option<String>,
  pub email:Option<String>,
  pub phonenumber:Option<String>,
  pub sex:Option<i8>,
  pub avatar:Option<String>,
  pub password:Option<String>,
  pub status:Option<i8>,
  pub del_flag:Option<i8>,
  pub login_ip:Option<String>,
  pub login_date:Option<String>,
  pub create_by:Option<String>,
  pub create_time:Option<String>,
  pub update_by:Option<String>,
  pub update_time:Option<String>,
  pub remark:Option<String>,
  pub real_name:Option<String>,
  pub expire_time:Option<i64>,
}