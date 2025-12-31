// Modify this so it can interact with the modules you set earlier
//
//

mod mysys;
mod table;

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
    if let Some(command) = args.get(1) {

        if args.len() > 2 {
            println!("Unknown option {} or too many options. Usage mysys cpu", command);
            return;
        }
    }
    
    else {
        println!("Usage: mysys cpu");
        return;
    }
    
    //
    //Run cpu module if checked usage passed
    //
    let mut table_items: Vec<Vec<String>> = Vec::new();
    let mut headers: Vec<(String, usize)> = Vec::new();

    match args[1].as_str() {
        
        "cpu" => {
            
            //Global CPU usage

            let header = (String::from("CPU Usage"), 2);

            let usages: Vec<String> = mysys::cpu::get_cpu_usages()
                .iter().map(|c| format!("{:.2}", c)).collect();

            let cpus: Vec<String> = usages.iter().enumerate()
                .map(|(i, _)| {
                    if i == 0 {
                        format!("Global")
                    } else {
                        format!("CPU {}", i)
                    }
                })
            .collect();

            table_items.push(cpus);
            table_items.push(usages);
            headers.push(header);

            Table::new(table_items, headers).print();

        }

        _ => {

            eprintln!("Unknown option: {}\nTry: cpu", args[1]);
        }
    }
}

//
// Print a nice square for some things
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
