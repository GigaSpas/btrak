use std::os::unix::process;
use std::process::exit;

use std::io;
use chrono::{self, Datelike};
use budget_manager::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let curr_date = chrono::Local::now().date_naive();
    
    if args.len() == 1 {
        check_files(&curr_date);
        let curr_month = read_month_file(curr_date).unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            exit(1)
        });

        let day = curr_month.get_day(curr_date.day() as u8);

        println!("{}", day.display(&true, &true, &vec![]));

        exit(0)

    }

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

                ");

        exit(0);

        }

    if args[1] == "add"{
        let mut date = curr_date;
        if args.len() == 3{
            date = chrono::NaiveDate::parse_from_str(&args[2], "%d.%m.%Y").expect("Could not parse the date make sure format is \"DD.MM.YYYY\"");
        }
        let mut month = read_month_file(date).unwrap_or(BudgetMonth::new(date.month0() as u8 + 1, Vec::new() ));
        let mut day = month.get_day(date.day0() as u8 + 1);

        let mut name = String::new();
        let mut desc= String::new();
        let mut tags_string= String::new();
        let mut money_string = String::new();

        println!("Enter name for the entry");
        io::stdin().read_line(&mut name).expect("Failed to read input");
        println!("Enter a description(optional)");
        io::stdin().read_line(&mut desc).expect("Failed to read input");
        println!("Enter tags(Comma seperated(exampl: shoping, luxury), optional)");
        io::stdin().read_line(&mut tags_string).expect("Failed to read input");
        println!("Enter the money spent(negative value) or earned(positive value) decimals should be seperated with \".\"");
        io::stdin().read_line(&mut money_string).expect("Failed to read input");

        let tags: Vec<String> = tags_string.trim().split(',').map(str::to_string).collect();
        let money: f32 = money_string.trim().parse::<f32>().expect("could not parse money");
        
        day = day.add_entrie(Entry::new(name.trim().to_string(), desc.trim().to_string(), tags, money));
        month = month.replace_entry(day);
        write_month_file(&date, &month);

        exit(0);

    }

    if args



    
}
