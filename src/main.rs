use sysinfo::{System, Disks};
use serde::Serialize;

// Define structures for JSON serialization
#[derive(Serialize)]
struct CpuInfo {
    index: usize,
    usage: f32,
}

#[derive(Serialize)]
struct DiskInfo {
    mount_point: String,
    total_gb: f64,
    available_gb: f64,
    used_gb: f64,
}

#[derive(Serialize)]
struct SystemInfo {
    cpu_count: usize,
    cpus: Vec<CpuInfo>,
    total_memory_gb: f64,
    used_memory_gb: f64,
    available_memory_gb: f64,
    memory_usage_percentage: f64,
    disks: Vec<DiskInfo>,
}

fn main() {
    // Create system instance
    let mut system = System::new_all();
    system.refresh_all();

    // Create Disks instance separately
    let disks = Disks::new_with_refreshed_list();

    // Collect CPU information
    let cpu_count = system.cpus().len();
    let cpus: Vec<CpuInfo> = system.cpus()
        .iter()
        .enumerate()
        .map(|(index, cpu)| CpuInfo {
            index,
            usage: cpu.cpu_usage(),
        })
        .collect();

    // Calculate memory values
    let total_memory_gb = system.total_memory() as f64 / 1_024.0 / 1_024.0 / 1_024.0;
    let used_memory_gb = system.used_memory() as f64 / 1_024.0 / 1_024.0 / 1_024.0;
    let available_memory_gb = total_memory_gb - used_memory_gb;
    let memory_usage_percentage = (used_memory_gb / total_memory_gb) * 100.0;

    // Collect disk information
    let disks: Vec<DiskInfo> = disks
        .iter()
        .map(|disk| {
            let total_gb = disk.total_space() as f64 / 1_000_000_000.0;
            let available_gb = disk.available_space() as f64 / 1_000_000_000.0;
            DiskInfo {
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total_gb,
                available_gb,
                used_gb: total_gb - available_gb,
            }
        })
        .collect();

    // Create the system info structure
    let system_info = SystemInfo {
        cpu_count,
        cpus,
        total_memory_gb,
        used_memory_gb,
        available_memory_gb,
        memory_usage_percentage,
        disks,
    };

    // Output as JSON
    match serde_json::to_string_pretty(&system_info) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}