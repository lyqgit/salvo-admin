use crate::entity::sys_user_entity::SysUserEntity;
use crate::entity::sys_captcha_entity::SysCaptchaEntity;
use rbatis::{crud,impl_select};

crud!(SysUserEntity{},"sys_user");

impl_select!(SysUserEntity{select_user_by_up(username:String,password:String)=>"`where user_name = #{username} and password = #{password} limit 1`"});


crud!(SysCaptchaEntity{},"sys_captcha");

impl_select!(SysCaptchaEntity{select_captcha_by_code(code:String)=>"`where code = #{code} limit 1`"});
