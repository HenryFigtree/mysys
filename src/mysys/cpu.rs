
// Small program to print system stats
//
use sysinfo::{System, RefreshKind, CpuRefreshKind};
use std::{thread, time::Duration};

pub fn get_cpu_usages() -> Vec<f32> {

    //
    //Create system with only cpu usage
    //
    let mut sys = System::new_with_specifics(RefreshKind::nothing().with_cpu(
            CpuRefreshKind::nothing().with_cpu_usage()));
    
    //
    // Need to refresh at least twice cpu state in a time interval
    //
    
    // First refresh initializes the cpu internal state
    sys.refresh_cpu_specifics(CpuRefreshKind::nothing().with_cpu_usage());
    
    //Wait a bit
    thread::sleep(Duration::from_millis(2000));
    
    // Refresh again
    sys.refresh_cpu_usage();
    
    //
    //Collect global cpu usage
    //
    let mut result = Vec::new();
    result.push(sys.global_cpu_usage());
    for cpu in sys.cpus() {
        result.push(cpu.cpu_usage());
    }

    result
}
