use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SysRoleUserEntity{
    pub role_id:i64,
    pub menu_id:i64,
}