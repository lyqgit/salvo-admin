use sysinfo::{ Disks, System };
use crate::model::monitor_model::{ Cpu,Mem,Sys,SysFiles };
use byte_unit::Byte;

pub fn get_cpu_info()->Cpu{
    let mut sys = System::new_all();
    sys.refresh_all();
    let global_cpu = sys.global_cpu_info();
    Cpu{ cpu_num:sys.cpus().len(), used: global_cpu.cpu_usage(), brand: global_cpu.name().to_string(), frequency: global_cpu.frequency() }
}

pub fn u64_to_gb(size:u64)->String{
    let byte = Byte::from_u64(size);
    format!("{byte:#.9}")
}

pub fn get_mem_info()->Mem{
    let mut sys = System::new_all();
    sys.refresh_all();
    Mem{
        total: u64_to_gb(sys.total_memory()),
        used: u64_to_gb(sys.used_memory()),
        total_swap: u64_to_gb(sys.total_swap()),
        used_swap: u64_to_gb(sys.used_swap()),
    }
}

pub fn get_sys_info()->Sys{
    let mut sys = System::new_all();
    sys.refresh_all();
    Sys{
        os_name: System::name(),
        os_version: System::os_version(),
        host_name: System::host_name(),
        kernel_version: System::kernel_version(),
    }
}

pub fn get_disk_info()->Vec<SysFiles>{
    let disks = Disks::new_with_refreshed_list();
    let mut sys_file_vec = Vec::<SysFiles>::new();
    for disk in &disks {
        sys_file_vec.push(SysFiles{
            name: disk.name().to_str().map_or(String::new(), |v| v.to_string()),
            sys_type_name: disk.file_system().to_str().map_or(String::new(), |v| v.to_string()),
            type_name: disk.kind().to_string(),
            total: u64_to_gb(disk.total_space()),
            free: u64_to_gb(disk.available_space()),
        })
    };
    sys_file_vec
}