use salvo::oapi::ToSchema;
use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
pub struct SysUser{
  #[serde(rename(serialize = "userId"))]
  pub user_id:i64,
  #[serde(rename(serialize = "deptId"))]
  pub dept_id:Option<i64>,
  #[serde(rename(serialize = "userName"))]
  pub user_name:String,
  #[serde(rename(serialize = "nickName"))]
  pub nick_name:String,
  #[serde(rename(serialize = "userType"))]
  pub user_type:Option<String>,
  pub email:Option<String>,
  pub phonenumber:Option<String>,
  pub sex:Option<String>,
  pub avatar:Option<String>,
  pub password:Option<String>,
  pub status:Option<String>,
  #[serde(rename(serialize = "delFlag"))]
  pub del_flag:Option<String>,
  #[serde(rename(serialize = "loginIp"))]
  pub login_ip:Option<String>,
  #[serde(rename(serialize = "loginDate"))]
  pub login_date:Option<DateTime>,
  #[serde(rename(serialize = "createBy"))]
  pub create_by:Option<String>,
  #[serde(rename(serialize = "createTime"))]
  pub create_time:Option<DateTime>,
  #[serde(rename(serialize = "updateBy"))]
  pub update_by:Option<String>,
  #[serde(rename(serialize = "updateTime"))]
  pub update_time:Option<DateTime>,
  pub remark:Option<String>,
  #[serde(rename(serialize = "realName"))]
  pub real_name:Option<String>,
  #[serde(rename(serialize = "expireTime"))]
  pub expire_time:Option<i64>,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SysUserEntity{
  pub user_id:Option<i64>,
  pub dept_id:Option<i64>,
  pub user_name:Option<String>,
  pub nick_name:Option<String>,
  pub user_type:Option<String>,
  pub email:Option<String>,
  #[serde(rename(serialize = "phonenumber"))]
  pub phone_number:Option<String>,
  pub sex:Option<String>,
  pub avatar:Option<String>,
  pub password:Option<String>,
  pub status:Option<String>,
  pub del_flag:Option<String>,
  pub login_ip:Option<String>,
  pub login_date:Option<DateTime>,
  pub create_by:Option<String>,
  pub create_time:Option<DateTime>,
  pub update_by:Option<String>,
  pub update_time:Option<DateTime>,
  pub remark:Option<String>,
  pub real_name:Option<String>,
  pub expire_time:Option<i64>,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct AddSysUserEntity{
  pub user_id:Option<i64>,
  pub dept_id:Option<i64>,
  pub user_name:Option<String>,
  pub nick_name:Option<String>,
  pub user_type:Option<String>,
  pub email:Option<String>,
  #[serde(rename(serialize = "phonenumber"))]
  pub phone_number:Option<String>,
  pub sex:Option<String>,
  pub avatar:Option<String>,
  pub password:Option<String>,
  pub status:Option<String>,
  pub del_flag:Option<String>,
  pub login_ip:Option<String>,
  pub login_date:Option<DateTime>,
  pub create_by:Option<String>,
  pub create_time:Option<DateTime>,
  pub update_by:Option<String>,
  pub update_time:Option<DateTime>,
  pub remark:Option<String>,
}