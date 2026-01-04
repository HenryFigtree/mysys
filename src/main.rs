//Light System monitor 
//
//

mod mysys;
mod table;

use sysinfo::{System, RefreshKind, CpuRefreshKind};
use std::{thread, time::Duration, process::exit};
use std::env::args;
use table::Table;

fn main() {
    //
    //Welcome title
    //

    square("Welcome to MySys", 20);
    println!("\n");
    
    //
    //Check command line arguments: Usage mysys arg1
    //
    let args: Vec<String> = args().collect();
    
    //
    //Check usage
    //
    
    if args.len() < 2 {
        println!("Usage: mysys [cpu]");
        return;
    }
    

    //
    //Add a new System
    //
    let mut sys = System::new();
    //Add refresh kind with all values set too false, cl args will set them true
    let mut refkind = RefreshKind::nothing();
    let mut show_cpu = false;

    //
    //Match args to set refresh kinds to true if matched
    //
    let mut headers: Vec<(String, usize)> = Vec::new();

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            
            "cpu" => {

                refkind = refkind.with_cpu(CpuRefreshKind::nothing().with_cpu_usage());
                headers.push((String::from("CPU Usage"), 2));
                show_cpu = true;
            }


            _ => {

                eprintln!("Unknown option: {}\nTry: cpu", arg);
                exit(1);
            }
        }
    }

    //refresh system with refkind values
    sys.refresh_specifics(refkind);

    //Refresh cpu usage again for accurate results if cpu is an arg
    if show_cpu {
        get_cpu_usages(&mut sys);
    }
    
    //
    //Format and Print table with system stats
    //
    let table_items = mysys::cpu::format_info(&sys);
    Table::new(table_items, headers).print();
}

//
//Functions
//

//
//Print a nice square for some things
//
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

//
//Refresh cpu usages after a time interval
//
fn get_cpu_usages(s: &mut System) {

    // Need to refresh at least twice cpu state in a time interval
    // If cpu is an argument then the first refresh is already done
    
    //Wait a bit
    thread::sleep(Duration::from_millis(2000));
    
    // Refresh again
    s.refresh_cpu_usage();
    
}
