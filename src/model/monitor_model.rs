use salvo::{oapi::{ToSchema}};
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,ToSchema,Deserialize,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct Cpu{
    pub cpu_num:usize,
    pub used:f32,
    pub brand:String,
    pub frequency:u64,
}

#[derive(Debug,Serialize,ToSchema,Deserialize,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct Mem{
    pub total:String,
    pub used:String,
    pub total_swap:String,
    pub used_swap:String,
}

#[derive(Debug,Serialize,ToSchema,Deserialize,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct Sys{
    pub os_name:Option<String>,
    pub os_version:Option<String>,
    pub host_name:Option<String>,
    pub kernel_version:Option<String>,
}

#[derive(Debug,Serialize,ToSchema,Deserialize,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct SysFiles{
    pub name:String, // 硬盘名称
    pub sys_type_name:String, // 文件系统
    pub type_name:String, // 盘符类型
    pub total:String,
    pub free:String,
}

#[derive(Debug,Serialize,ToSchema,Deserialize,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct ServerInfo{
    pub cpu:Cpu,
    pub mem:Mem,
    pub sys:Sys,
    pub sys_files:Vec<SysFiles>
}
