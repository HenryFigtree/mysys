
// Fortmats the system stats
//
use sysinfo::{System, Disk, Networks};

pub fn format_cpu(s: &System) -> Vec<Vec<String>> {
    
    let mut table_cols: Vec<Vec<String>> = Vec::new();
    //
    //Collect system info 
    //
    let mut result = Vec::new();
    result.push(s.global_cpu_usage());
    for cpu in s.cpus() {
        result.push(cpu.cpu_usage());
    }

    let usages: Vec<String> = result 
        .iter().map(|c| format!("{:.2}%", c)).collect();

    let cpus: Vec<String> = usages.iter().enumerate()
        .map(|(i, _)| {
            if i == 0 {
                format!("Global")
            } else {
                format!("CPU {}", i - 1)
                   }
        })
          .collect();

    table_cols.push(cpus);
    table_cols.push(usages);
    table_cols
}

pub fn format_ram(s: &System) -> Vec<Vec<String>> {
    let mut table_cols: Vec<Vec<String>> = Vec::new();
    let mut result = Vec::new();
    result.push(s.total_memory() as f64 / 1024.0_f64.powi(3));
    result.push(s.used_memory() as f64 / 1024.0_f64.powi(3));
    result.push(s.free_memory() as f64 / 1024.0_f64.powi(3));
    result.push(s.total_swap() as f64 / 1024.0_f64.powi(3));
    result.push(s.used_swap() as f64 / 1024.0_f64.powi(3));
    result.push(s.free_swap() as f64 / 1024.0_f64.powi(3));

    let usages: Vec<String> = result 
        .iter().map(|c| format!("{:.2} GiB", c)).collect();

    let mut titles: Vec<String> = Vec::new();
    titles.push(String::from("Total"));
    titles.push(String::from("Used"));
    titles.push(String::from("Free"));
    titles.push(String::from("Total swap"));
    titles.push(String::from("Used swap"));
    titles.push(String::from("Free swap"));

    table_cols.push(titles);
    table_cols.push(usages);
    table_cols
}

pub fn disk_titles() -> Vec<Vec<String>> {
    let mut table_cols: Vec<Vec<String>> = Vec::new();
    let mut titles: Vec<String> = Vec::new();
    
    titles.push(String::from("Disk name"));
    titles.push(String::from("Kind"));
    titles.push(String::from("Mount point"));
    titles.push(String::from("Filesystem"));
    titles.push(String::from("Total space"));
    titles.push(String::from("Available space"));

    table_cols.push(titles);
    table_cols


}

pub fn format_disks(d: &Disk) -> Vec<Vec<String>> {
    let mut table_cols: Vec<Vec<String>> = Vec::new();
    let mut result = Vec::new();
    
    result.push(format!("{}", d.name().to_string_lossy().into_owned()));
    result.push(d.kind().to_string());
    result.push(format!("{}", d.mount_point().to_string_lossy().into_owned()));
    result.push(format!("{}", d.file_system().to_string_lossy().into_owned()));
    result.push(format!("{:.2} GiB", d.total_space() as f64 / 1024.0_f64.powi(3)));
    result.push(format!("{:.2} GiB", d.available_space() as f64 / 1024.0_f64.powi(3)));

    table_cols.push(result);
    table_cols

}

pub fn network_titles() -> Vec<Vec<String>> {
    let mut table_cols: Vec<Vec<String>> = Vec::new();
    let mut titles: Vec<String> = Vec::new();
    
    titles.push(String::from("Interface"));
    titles.push(String::from("MAC"));
    titles.push(String::from("MTU"));
    titles.push(String::from("Received"));
    titles.push(String::from("Total Received"));
    titles.push(String::from("Transmitted"));
    titles.push(String::from("Total Transmitted"));
    titles.push(String::from("Packets Received"));
    titles.push(String::from("Total packets Received"));
    titles.push(String::from("Packets transmitted"));
    titles.push(String::from("Total packets transmitted"));
    titles.push(String::from("Errors on Received"));
    titles.push(String::from("Total errors on Received"));
    titles.push(String::from("Errors on  transmitted"));
    titles.push(String::from("Total errors on transmitted"));

    table_cols.push(titles);
    table_cols


}


pub fn format_networks(n: &Networks) -> Vec<Vec<String>> {
    let mut table_cols: Vec<Vec<String>> = Vec::new();
   
    for (interface, network) in n.list() {
        let mut result = Vec::new();
        result.push(format!("{}",interface));
        result.push(format!("{}", network.mac_address()));
        result.push(format!("{} B", network.mtu()));
        result.push(format!("{} B", network.received()));
        result.push(format!("{} B", network.total_received()));
        result.push(format!("{} B", network.transmitted()));
        result.push(format!("{} B", network.total_transmitted()));
        result.push(format!("{} B", network.packets_received()));
        result.push(format!("{} B", network.total_packets_received()));
        result.push(format!("{} B", network.packets_transmitted()));
        result.push(format!("{} B", network.total_packets_transmitted()));
        result.push(format!("{} B", network.errors_on_received()));
        result.push(format!("{} B", network.total_errors_on_received()));
        result.push(format!("{} B", network.errors_on_transmitted()));
        result.push(format!("{} B", network.total_errors_on_transmitted()));
        table_cols.push(result);
        }

    table_cols

}

pub fn format_ip(n: &Networks) -> Vec<Vec<String>> {
    let mut table_cols: Vec<Vec<String>> = Vec::new();

    for (interface, network) in n.list() {
        let mut result = Vec::new();
        result.push(format!("{}", interface));
        for ip in network.ip_networks() {
            result.push(format!("{}", ip));
        }
        table_cols.push(result);
    }

    table_cols
}
