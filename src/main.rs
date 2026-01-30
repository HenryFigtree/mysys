//Light System monitor 
//
//

mod mysys;
mod table;

use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind, Disks, Networks};
use crossterm::terminal::size;
use std::{thread, time::Duration, process::exit};
use std::env::args;
use table::Table;

struct Config {
    cpu: bool,
    ram: bool,
    disks: bool,
    network: bool,
}

//Configure Config according to args passed by user
fn parse_args() -> Config {
    let mut config = Config {
        cpu: false,
        ram: false,
        disks: false,
        network: false,
    };

    let mut args = args().skip(1);
    let first = match args.next() {
        Some(arg) => arg,
        None => {
            println!("Usage: mysys [cpu] [ram] [disks] [network]");
            println!("Use flag --help for more information");
            exit(0);
        }
    };

    match first.as_str() {
        "--help" => {
            help();
            exit(0);
        }
        
        "cpu" => {

            config.cpu = true;
        }
        "ram" => {
            config.ram = true;
        }
        "disks" => {
            config.disks = true;
        }

        "network" => {
            config.network = true;
        }


        _ => {

            eprintln!("Unknown option: \nTry: cpu ram disks network");
            exit(1);
        }
    }

    //first consumes first argument. match the rest.

    for arg in args{
        match arg.as_str() {
            "--help" => {
                help();
                exit(0);
            }
            
            "cpu" => {

                config.cpu = true;
            }
            "ram" => {
                config.ram = true;
            }
            "disks" => {
                config.disks = true;
            }

            "network" => {
                config.network = true;
            }


            _ => {

                eprintln!("Unknown option: {}\nTry: cpu ram disks network", arg);
                exit(1);
            }
        }
    }
    config
}

fn help() {
    println!("
Usage: mysys [cpu] [ram] [disks] [network]
You can write one, two or all arguments to display a table with the information requested.\n
cpu: Displays the individual usage of your logical CPUs as reported to the OS as well as the sum of all of them in the Golbal CPU usage.\n
ram: Displays physical ram information such as total, free, available memory as well as total, free and available memory.\n
disks: Displays information about your disks. On Linux it appears it shows partitions and not disks e.g. it will show sda1, sda2, etc. instead of sda.\n
network: Shows network data transfers in bytes and packets as well as other information like IP address, MAC address; all the information is per interface.\n
        ")
}

fn main() {

    //Check command line arguments: Usage mysys arg1
    let config = parse_args();

    //---------------+
    // Welcome title |
    //---------------+
    square("Welcome to MySys", 20);
    print!("\n");
    println!("This is a small system monitor that prints system stats from cpu, ram, disks and your networks");

    //------------------+
    // Add a new System |
    //------------------+
    let mut sys = System::new();
    
    //-------------+
    // Format Data |
    //-------------+
    let mut headers: Vec<(String, usize)> = Vec::new();
    let mut table_items: Vec<Vec<String>> = Vec::new();

    //Refresh cpu usage again and format CPU
    if config.cpu {
        sys.refresh_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::nothing().with_cpu_usage()));
        headers.push((String::from("CPU Usage"), 2));
        get_cpu_usages(&mut sys);
        table_items.extend(mysys::format::format_cpu(&sys));
    }
    //Format RAM
    if config.ram {
        sys.refresh_specifics(RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()));
        headers.push((String::from("Memory"),2));
        table_items.extend(mysys::format::format_ram(&sys));
    }
    //Create disks and format data
    if config.disks {
        let disks = Disks::new_with_refreshed_list();
        let disk_cols = disks.list().len() + 1;
        headers.push((String::from("Disks"), disk_cols));
        table_items.extend(mysys::format::disk_titles());

        for disk in disks.list() {
            table_items.extend(mysys::format::format_disks(&disk));
        }
    }
    //Format network
    if config.network {
        let networks = Networks::new_with_refreshed_list();
        headers.push((String::from("Network"), networks.len() + 1));
        table_items.extend(mysys::format::network_titles());
        table_items.extend(mysys::format::format_networks(&networks));
        headers.push((String::from("IPs per interface"), networks.len()));
        table_items.extend(mysys::format::format_ip(&networks));
    }

    //-------------------------------+
    // Print table with system stats |
    //-------------------------------+
    let table = Table::new(table_items, headers);
    if Terminal::size() < table.table_size() {

    }
}

fn square(text: &str, width: usize) {

    let line = || {
        println!("+{}+", "-".repeat(width));
    };
    
    //
    //Centering
    //
    let padding = if text.len() < width {
        width - text.len()
    } else {
        0
    };

    let left = padding / 2;
    let right = padding - left;

    line();
    println!("|{}{}{}|", " ".repeat(left), text, " ".repeat(right));
    line();

}

//------------------------------------------+
// Refresh cpu usages after a time interval |
//------------------------------------------+
fn get_cpu_usages(s: &mut System) {

    // Need to refresh at least twice cpu state in a time interval
    // If cpu is an argument then the first refresh is already done
    
    //Wait a bit
    thread::sleep(Duration::from_millis(2000));
    
    // Refresh again
    s.refresh_cpu_usage();
    
}
