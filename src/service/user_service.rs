use crate::entity::sys_user_entity::SysUserEntity;
use crate::entity::sys_captcha_entity::SysCaptchaEntity;
use crate::GLOBAL_DB;
use rbatis::rbdc::Error;

pub async fn get_user_by_up(username:String,password:String)->Result<Option<SysUserEntity>,Error>{
  match SysUserEntity::select_user_by_up(&mut GLOBAL_DB.clone(), username, password).await{
    Ok(list)=>{
      Ok(list.get(0).cloned())
    },
    Err(err)=>Err(err)
  }
}

pub async fn get_captcha_by_code(code:String)->Result<Option<SysCaptchaEntity>,Error>{
  match SysCaptchaEntity::select_captcha_by_code(&mut GLOBAL_DB.clone(), code).await{
    Ok(list)=>{
      Ok(list.get(0).cloned())
    },
    Err(err)=>Err(err)
  }
}