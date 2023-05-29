use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SysUserPostEntity {
    pub user_id: i64,
    pub post_id: i64,
}