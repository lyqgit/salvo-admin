use rbatis::rbdc::datetime::DateTime;
use salvo::{oapi::{ToSchema}};
use salvo::oapi::ToParameters;
use serde::{Serialize,Deserialize};
use crate::entity::sys_user_entity::SysUser;
use crate::model::dept_model::DeptList;
use crate::model::post_model::SysPostList;
use crate::model::role_model::SysRoleList;

// 验证码返回
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[schema(rename_all="camelCase")]
pub struct CaptchaRes{
  #[serde(rename="captchaEnabled")]
  pub captcha_enabled:Option<bool>,
  pub img:String,
  pub uuid:String
}

// 登录请求参数
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct LoginReq{
  pub code:Option<String>,
  pub password:Option<String>,
  pub username:Option<String>,
  pub uuid:Option<String>
}


// 登录返回
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct LoginRes{
  pub token:String,
}

// 登录返回
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct UserInfo{
  pub permissions:Vec<String>,
  pub roles:Vec<String>,
  pub user:SysUser
}


#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[schema(rename_all="camelCase")]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysUserList{
  pub user_id:i64,
  pub dept_id:Option<i64>,
  pub user_name:String,
  pub nick_name:String,
  pub user_type:Option<String>,
  pub email:Option<String>,
  #[serde(rename(serialize = "phonenumber"))]
  #[serde(rename(deserialize = "phonenumber"))]
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
  pub dept:Option<DeptList>
}

#[derive(Debug,Serialize,Deserialize,Clone,ToParameters)]
#[parameters(rename_all="camelCase")]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysUserListPayload{
  pub page_num:u64,
  pub page_size:u64,
  pub user_name:Option<String>,
  #[serde(rename(deserialize = "phonenumber"))]
  pub phone_number:Option<String>,
  pub status:Option<String>,
  pub dept_id:Option<i64>,
  #[serde(rename(deserialize ="params[beginTime]"))]
  #[parameter(rename="params[beginTime]")]
  #[parameter(value_type = Option<String>)]
  pub begin_time:Option<DateTime>,
  #[serde(rename(deserialize ="params[endTime]"))]
  #[parameter(rename="params[endTime]")]
  #[parameter(value_type = Option<String>)]
  pub end_time:Option<DateTime>,
}

#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[schema(rename_all="camelCase")]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysUserDetail{
  pub posts:Vec<SysPostList>,
  pub roles:Vec<SysRoleList>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub post_ids:Option<Vec<i64>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub role_ids:Option<Vec<i64>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user:Option<SysUserList>
}
