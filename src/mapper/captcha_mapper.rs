use crate::entity::sys_captcha_entity::SysCaptcha;
use rbatis::{crud,impl_select};

crud!(SysCaptcha{},"sys_captcha");

impl_select!(SysCaptcha{select_captcha_by_code(code:String)=>"`where code = #{code} limit 1`"});