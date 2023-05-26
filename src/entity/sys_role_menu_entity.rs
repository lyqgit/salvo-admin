use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SysRoleMenuEntity{
    pub role_id:i64,
    pub menu_id:i64,
}