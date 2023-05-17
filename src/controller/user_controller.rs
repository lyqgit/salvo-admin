use crate::utils::captcha;
use salvo::Depot;
use salvo::oapi::extract::JsonBody;
use salvo::{oapi::endpoint};
use crate::model::user_model::{CaptchaRes,LoginReq,LoginRes,UserInfo};
use crate::utils::res::{Res,res_json_ok,res_json_err,ResObj,res_json_custom};
use uuid::Uuid;
use crate::service::user_service::{get_user_by_up,get_user_by_id};
use crate::service::role_service;
use crate::service::menu_service;
use crate::utils::webtoken::create_token;
use crate::utils::md5::create_md5;
use crate::utils::redis;

#[endpoint(
  responses(
    (status = 200,body=ResObj<CaptchaRes>)
  ),
)]
pub async fn get_captcha()->Res<CaptchaRes>{
  if let (_,Some(base64)) = captcha::create_captcha(){
    let uuid = Uuid::new_v4().to_string();
    redis::set_ex(&uuid, &base64, 300).unwrap();
    Ok(res_json_ok(Some(CaptchaRes{img:base64,captcha_enabled:Some(true),uuid})))
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
  if let Some(_) = login_body.code.clone(){
    let captcha:String = redis::get(login_body.uuid.clone()).unwrap();
    if captcha.is_empty(){
      Err(res_json_err("验证码错误".to_string()))
    }else{
      redis::del(login_body.uuid.clone()).unwrap();
      if let (Some(username),Some(mut password)) = (login_body.username.clone(),login_body.password.clone()){
        password.push_str("salvo_admin");
        match get_user_by_up(username,create_md5(password)).await{
          Ok(r_op)=>{
            if let Some(user) = r_op{
              match create_token(user.user_id.try_into().unwrap(),user.user_name) {
                  Ok(token)=>{
                    redis::set_ex(&token,user.user_id,3600).unwrap();
                    Ok(res_json_ok(Some(LoginRes{token})))
                  },
                  Err(err)=>{
                    tracing::error!("{}", err.to_string());
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
        Err(res_json_custom(400,"用户账号或密码必须填写".to_string()))
      }
    }
  }else{
    Err(res_json_err("验证码错误".to_string()))
  }
}

#[endpoint(
  responses(
    (status = 200,body=ResObj<UserInfo>)
  ),
)]
pub async fn get_info(depot: &mut Depot)->Res<UserInfo>{
  let user_id = depot.get::<i32>("userId").copied().unwrap();
  match get_user_by_id(user_id).await{
    Ok(user_op)=>{
      if let Some(user) = user_op{
        match role_service::get_roles_by_user_id(user_id).await{
          Ok(role_list)=>{
            if user.user_name.eq("admin"){
              let user_info = UserInfo{user,roles:Vec::new(),permissions:vec!["*:*:*".to_string()]};
              Ok(res_json_ok(Some(user_info)))
            }else{
              let permissions = menu_service::get_menu_by_role_id(true,role_list.join(",")).await.unwrap();
              let user_info = UserInfo{user,roles:role_list,permissions};
              Ok(res_json_ok(Some(user_info)))
            }
          },
          Err(err)=>{
            Err(res_json_custom(400,err.to_string()))
          }
        }

      }else{
        Err(res_json_custom(400,"用户不存在".to_string()))
      }
    },
    Err(err)=>{
      Err(res_json_custom(400,err.to_string()))
    }
  }
}
