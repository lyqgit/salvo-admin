use crate::entity::sys_user_entity::SysUser;
use rbatis::{crud,impl_select};

crud!(SysUser{},"sys_user");

impl_select!(SysUser{select_user_by_up(username:String,password:String)=>"`where user_name = #{username} and password = #{password} limit 1`"});




