use crate::utils::captcha;
use salvo::Depot;
use salvo::Request;
use salvo::oapi::extract::{JsonBody, PathParam};
use salvo::{oapi::endpoint};
use crate::model::user_model::{CaptchaRes, LoginReq, LoginRes, SysAuthPayload, SysUserAuthRole, SysUserChangeStatusPayload, SysUserDetail, SysUserEditPayload, SysUserEditPwdPayload, SysUserList, SysUserListPayload, SysUserModifyPayload, UserInfo};
use crate::utils::res::{Res, res_json_ok, res_json_err, ResObj, res_json_custom, match_ok, match_no_res_ok};
use uuid::Uuid;
use crate::model::common_model::Page;
use crate::service::user_service::{get_user_by_up,get_user_by_id};
use crate::service::{role_service, user_service};
use crate::service::menu_service;
use crate::utils::webtoken::create_token;
use crate::utils::md5::create_md5;
use crate::utils::redis;
use crate::model::menu_model::Router;

/// 获取验证码
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<CaptchaRes>,description ="获取验证码")
  ),
)]
pub async fn get_captcha()->Res<CaptchaRes>{
  if let (captcha_str,Some(base64)) = captcha::create_captcha(){
    let uuid = Uuid::new_v4().to_string();
    // 验证码转小写
    redis::set_ex(&uuid, captcha_str.to_lowercase(), 300).unwrap();
    Ok(res_json_ok(Some(CaptchaRes{img:base64,captcha_enabled:Some(true),uuid})))
  }else{
    Err(res_json_err("验证码生成失败".to_string()))
  }
}

/// 登录
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<LoginRes>,description ="登录")
  ),
)]
pub async fn login(login_body:JsonBody<LoginReq>)->Res<LoginRes>{
  if let Some(captcha_str) = login_body.code.clone(){
    let captcha:String = redis::get(login_body.uuid.clone()).map_or(String::new(),|v|v);
    if captcha.is_empty() || !captcha_str.eq(&captcha){
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
                  Err(_err)=>{
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

/// 获取用户信息
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<UserInfo>,description ="获取用户信息")
  ),
)]
pub async fn get_info(depot: &mut Depot)->Res<UserInfo>{
  let user_id = depot.get::<i32>("userId").copied().unwrap();
  match get_user_by_id(user_id).await{
    Ok(user_op)=>{
      if let Some(user) = user_op{
        match role_service::get_roles_by_user_id(user_id).await{
          Ok(role_list)=>{
            if role_list.contains(&String::from("admin")){
              let user_info = UserInfo{user,roles:vec![String::from("admin")],permissions:vec!["*:*:*".to_string()]};
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

/// 获取用户路由
#[endpoint(
  tags("路由"),
  responses(
    (status_code = 200,body=ResObj<Vec<Router>>,description ="获取用户路由")
  ),
)]
pub async fn get_routers(depot: &mut Depot)->Res<Vec<Router>>{
  let user_id = depot.get::<i32>("userId").copied().unwrap();
  // Ok(res_json_ok(Some(user_id)))
  match role_service::get_roles_by_user_id(user_id).await{
    Ok(role_list)=>{
      if role_list.contains(&String::from("admin")){
        menu_service::get_router_tree(true, user_id).await.map_or_else(|err|{
          Err(res_json_custom(400,err.to_string()))
        }, |v|{
          Ok(res_json_ok(Some(v)))
        })
        
      }else{
        menu_service::get_router_tree(false, user_id).await.map_or_else(|err|{
          Err(res_json_custom(400,err.to_string()))
        }, |v|{
          Ok(res_json_ok(Some(v)))
        })
      }
    },
    Err(err)=>{
      Err(res_json_custom(400,err.to_string()))
    }
  }
}

/// 退出登录
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<()>,description ="退出登录")
  ),
)]
pub async fn log_out(req:&mut Request)->Res<()>{
  if let Some(token) = req.headers().get("Authorization"){
    match redis::del(token.to_str().map_or("",|v|v).to_string().replace("Bearer ","")){
      _=>Ok(res_json_ok(None))
    }
  }else{
    Ok(res_json_custom(401,"用户无权限".to_string()))
  }
}

/// 用户列表
#[endpoint(
  tags("用户"),
  parameters(
    SysUserListPayload
  ),
  responses(
    (status_code = 200,body=ResObj<Page<SysUserList>>,description ="用户列表")
  ),
)]
pub async fn get_user_page(req:&mut Request)->Res<Page<SysUserList>>{
  let payload = req.parse_queries().map_or(SysUserListPayload{ page_num: 1, page_size: 10, user_name: None, phone_number: None, status: None, dept_id: None, begin_time:None,end_time:None }, |v|v);
  match_ok(user_service::get_user_page(payload.page_num,payload.page_size,payload.user_name.clone(),payload.phone_number.clone(),payload.status.clone(),payload.begin_time,payload.end_time,payload.dept_id).await)
}


/// 用户详情
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<SysUserDetail>,description ="用户详情")
  ),
)]
pub async fn get_user_detail(id:PathParam<Option<i64>>)->Res<SysUserDetail>{
  match_ok(user_service::get_detail_by_id(id.into_inner()).await)
}


/// 删除用户
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<()>,description ="删除用户")
  ),
)]
pub async fn del_user(id:PathParam<i64>)->Res<()>{
  match_no_res_ok(user_service::del_user(id.into_inner()).await)
}

/// 修改用户密码
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<()>,description ="修改用户密码")
  ),
)]
pub async fn update_user_pwd(payload:JsonBody<SysUserEditPwdPayload>)->Res<()>{
  match_no_res_ok(user_service::update_user_pwd(
    payload.user_id,
    create_md5(payload.password.clone())
  ).await)
}


/// 获取部门和角色列表
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<SysUserDetail>,description ="获取部门和角色列表")
  ),
)]
pub async fn get_dept_and_role()->Res<SysUserDetail>{
  match_ok(user_service::get_detail_by_id(None).await)
}


/// 修改用户状态
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<()>,description ="修改用户状态")
  ),
)]
pub async fn put_change_status_by_id(payload:JsonBody<SysUserChangeStatusPayload>)->Res<()>{
  match_no_res_ok(user_service::update_user_status_by_id(payload.status.clone(),payload.user_id).await)
}

/// 添加用户
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<()>,description ="添加用户")
  ),
)]
pub async fn post_add_user(payload:JsonBody<SysUserModifyPayload>,depot:&mut Depot)->Res<()>{
  let user_id = depot.get::<i32>("userId").copied().unwrap();
  let mut password = payload.password.clone();
  password.push_str("salvo_admin");
  match_no_res_ok(user_service::add_user(
    user_id,
    payload.dept_id,
    payload.email.clone(),
    payload.nick_name.clone(),
    payload.user_name.clone(),
    create_md5(password),
    payload.status.clone(),
    payload.sex.clone(),
    payload.phone_number.clone(),
    payload.post_ids.clone(),
    payload.role_ids.clone(),
    payload.remark.clone()
  ).await)
}

/// 修改用户
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<()>,description ="修改用户")
  ),
)]
pub async fn put_edit_user(payload:JsonBody<SysUserEditPayload>,depot:&mut Depot)->Res<()>{
  let user_id = depot.get::<i32>("userId").copied().unwrap();
  match_no_res_ok(user_service::edit_user(
    user_id,
    payload.user_id,
    payload.dept_id,
    payload.email.clone(),
    payload.nick_name.clone(),
    payload.status.clone(),
    payload.sex.clone(),
    payload.phone_number.clone(),
    payload.post_ids.clone(),
    payload.role_ids.clone(),
    payload.remark.clone()
  ).await)
}


/// 查询用户详情和角色列表
#[endpoint(
  tags("用户"),
  responses(
    (status_code = 200,body=ResObj<SysUserAuthRole>,description ="查询用户详情和角色列表")
  ),
)]
pub async fn get_user_auth_role_by_id(id:PathParam<i64>)->Res<SysUserAuthRole>{
  match_ok(user_service::get_user_auth_role_by_id(id.into_inner()).await)
}

/// 修改用户权限
#[endpoint(
  tags("用户"),
  parameters(
    SysAuthPayload
  ),
  responses(
    (status_code = 200,body=ResObj<()>,description ="修改用户权限")
  ),
)]
pub async fn add_user_and_role(req:&mut Request)->Res<()>{
  let user_role:SysAuthPayload = req.parse_queries().unwrap();
  let user_id = user_role.user_id;
  match user_role.role_ids {
    Some(v)=>{
      let role_ids_str:Vec<&str> = v.split(",").collect::<Vec<&str>>();
      let role_ids = role_ids_str.into_iter().map(|it|it.parse::<i64>().unwrap()).collect::<Vec<i64>>();
          match_no_res_ok(user_service::add_user_and_role(user_id,Some(role_ids)).await)
    },
    None=>{
      match_no_res_ok(user_service::add_user_and_role(user_id,None).await)
    }
  }

}
