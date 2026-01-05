
// Fortmats the system stats
//
use sysinfo::{System};

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
                format!("CPU {}", i)
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
