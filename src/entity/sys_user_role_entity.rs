use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SysUserRoleEntity {
    pub user_id: i64,
    pub role_id: i64,
}