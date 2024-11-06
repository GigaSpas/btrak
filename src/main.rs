use std::process;
use chrono::{self, Datelike};

use budget_manager::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let curr_date = chrono::Local::now().date_naive();
    
    if args.len() == 1 {
        check_files(&curr_date);
        let curr_month = read_month_file(curr_date).unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            process::exit(1)
        });

        let day = curr_month.get_day(curr_date.day() as u8);

        day.display(&true, &true, &vec![]);

        process::exit(0)

    }

    if args.len() == 2 {
        if args[1] == "-h" || args[1] == "--help" {
            println!("
A simple budget tracking program

Usage: btrak [command] [options] [date]
The date should be formated \"DD.MM.YYYY\" and defaults to today if it is not provided

Commands:
    a add       Adds an entry
    d delete    Delete an entry
    p print     Print an entry to stdout (Default behavior)

Options:
    -d --day
    -m --month
    -t --tags
    -e --expenses

                ")
        }

    }


    
}
