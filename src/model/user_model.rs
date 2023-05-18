use salvo::{oapi::{ToSchema}};
use serde::{Serialize,Deserialize};
use crate::entity::sys_user_entity::SysUser;

// 验证码返回
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
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


