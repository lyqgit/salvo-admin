use salvo::oapi::{ToSchema};
use serde::{Serialize,Deserialize};

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
  pub code:String,
  pub password:String,
  pub username:String,
  pub uuid:String
}


// 登录返回
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct LoginRes{
  pub token:String,
}

