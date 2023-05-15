use crate::utils::captcha;
use salvo::oapi::extract::JsonBody;
use salvo::{oapi::endpoint};
use crate::model::user_model::{CaptchaRes,LoginReq,LoginRes};
use crate::utils::res::{Res,res_json_ok,res_json_err,ResObj,res_json_custom};
use uuid::Uuid;
use crate::service::user_service::{get_captcha_by_code,get_user_by_up};
use crate::utils::webtoken::create_token;
use crate::GLOBAL_REDIS;
use redis::{Commands, Client};

#[endpoint(
  responses(
    (status = 200,body=ResObj<CaptchaRes>)
  ),
)]
pub async fn get_captcha()->Res<CaptchaRes>{
  if let (_,Some(base64)) = captcha::create_captcha(){
    Ok(res_json_ok(Some(CaptchaRes{img:base64,captcha_enabled:Some(true),uuid:Uuid::new_v4().to_string()})))
  }else{
    Err(res_json_err("验证码生成失败".to_string()))
  }

}

#[endpoint(
  responses(
    (status = 200,body=ResObj<LoginRes>)
  ),
)]
pub async fn login(login_body:JsonBody<LoginReq>)->Res<LoginRes>{
  match get_captcha_by_code(login_body.code.clone()).await{
    Ok(captcha_one_option)=>{
      if let Some(_) = captcha_one_option{
        match get_user_by_up(login_body.username.clone(),login_body.password.clone()).await{
          Ok(r_op)=>{
            if let Some(user) = r_op{
              match create_token(user.user_id.try_into().unwrap(),user.user_name) {
                  Ok(token)=>{
                    let _:() = Client::set_ex(&mut GLOBAL_REDIS.clone(),user.user_id, &token, 3600).unwrap();
                    Ok(res_json_ok(Some(LoginRes{token})))
                  },
                  _=>{
                    Err(res_json_custom(400,"token生成失败".to_string()))
                  }
              }
            }else{
              Err(res_json_custom(400,"用户不存在".to_string()))
            }
          },
          Err(err)=>Err(res_json_err(err.to_string()))
        }
      }else{
        Err(res_json_err("验证码错误".to_string()))
      }
    },
    _=>Err(res_json_err("验证码错误".to_string()))
  }
}
