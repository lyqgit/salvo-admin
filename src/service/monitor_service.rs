use crate::utils::system;
use crate::model::monitor_model::ServerInfo;

pub async fn get_sys_info()->Result<ServerInfo,()>{
    Ok(ServerInfo{
        cpu:system::get_cpu_info(),
        mem:system::get_mem_info(),
        sys:system::get_sys_info(),
        sys_files: system::get_disk_info(),
    })
}