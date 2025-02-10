use std::process::exit;

use budget_manager::*;
use chrono::{self, Datelike};
use std::io;

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

        println!("{}", day.display(&true, &true, &true, &vec![]));
        exit(0)
    }

    if args[1] == "-h" || args[1] == "--help" {
        println!(
            "
A simple budget tracking program

Usage: btrak [command] [options] [date]
The date should be formated \"DD.MM.YYYY\" and defaults to today if it is not provided

Files are saved in Documents folder

Examples:
btrak
btrak 20.10.2025
btrak display -de
btrak -dte 12.10.2024 
btrak add
btrak remove

Commands:
    add       Adds an entry
    remove    Delete an entry
    display   Print an entry to stdout (Default behavior)

Options:
    -d        Display day (Default behavior)
    -m        Display month
    -t        Filter by tags
    -i        Don't display income
    -e        Don't display expenses

                "
        );

        exit(0);
    }

    if args[1] == "display" {
        if args.len() == 2 {
            check_files(&curr_date);
            let curr_month = read_month_file(curr_date).unwrap_or_else(|e| {
                eprintln!("Error: {e}");
                exit(1)
            });

            let day = curr_month.get_day(curr_date.day() as u8);

            println!("{}", day.display(&true, &true, &true, &vec![]));

            exit(0)
        }

        if args.len() == 3 {
            let mut tags: Vec<String> = Vec::new();
            let mut income: bool = false;
            let mut expenses: bool = true;
            let mut display_day: bool = true;

            if args[2].starts_with('-') {
                if args[2].contains('m') {
                    display_day = false;
                }
                if args[2].contains("d") {
                    display_day = true;
                }
                if args[2].contains('t') {
                    println!("Input space \" \" separated tags");
                    let mut tags_string = String::new();
                    io::stdin()
                        .read_line(&mut tags_string)
                        .expect("Failed to read input");
                    tags = tags_string.trim().split(' ').map(str::to_string).collect();
                }
                if args[2].contains('i') {
                    income = false;
                }
                if args[2].contains('e') {
                    expenses = false;
                }
            }
            let date = curr_date;
            let month = read_month_file(date)
                .unwrap_or(BudgetMonth::new(date.month0() as u8 + 1, Vec::new()));
            if !display_day {
                println!(
                    "{}",
                    month.display(date.month0() as u32 + 1, &expenses, &income, &true, &tags)
                );
                exit(0)
            }
            let day = month.get_day(date.day0() as u8 + 1);
            println!("{}", day.display(&expenses, &income, &true, &tags));
            exit(0)
        }
        if args.len() == 4 {
            let mut tags: Vec<String> = Vec::new();
            let mut income: bool = true;
            let mut expenses: bool = true;
            let mut display_day: bool = true;

            if args[2].starts_with('-') {
                if args[2].contains('m') {
                    display_day = false;
                }
                if args[2].contains("d") {
                    display_day = true;
                }
                if args[2].contains('t') {
                    println!("Input space \" \" separated tags");
                    let mut tags_string = String::new();
                    io::stdin()
                        .read_line(&mut tags_string)
                        .expect("Failed to read input");
                    tags = tags_string.trim().split(' ').map(str::to_string).collect();
                }
                if args[2].contains('i') {
                    income = false;
                }
                if args[2].contains('e') {
                    expenses = false;
                }
            }
            let date;
            date = chrono::NaiveDate::parse_from_str(&args[3], "%d.%m.%Y")
                .expect("Could not parse the date make sure format is \"DD.MM.YYYY\"");
            let month = read_month_file(date)
                .unwrap_or(BudgetMonth::new(date.month0() as u8 + 1, Vec::new()));
            if !display_day {
                println!(
                    "{}",
                    month.display(date.month0() as u32 + 1, &expenses, &income, &true, &tags)
                );
                exit(0)
            }
            let day = month.get_day(date.day0() as u8 + 1);
            println!("{}", day.display(&expenses, &income, &true, &tags));
            exit(0)
        }
    }
    if args[1].starts_with('-') {
        if args.len() == 3 {
            let mut tags: Vec<String> = Vec::new();

            let mut income: bool = true;
            let mut expenses: bool = true;
            let mut display_day: bool = true;

            if args[1].contains('m') {
                display_day = false;
            }
            if args[1].contains("d") {
                display_day = true;
            }
            if args[1].contains('t') {
                println!("Input space \" \" separated tags");
                let mut tags_string = String::new();
                io::stdin()
                    .read_line(&mut tags_string)
                    .expect("Failed to read input");
                tags = tags_string.trim().split(' ').map(str::to_string).collect();
            }
            if args[1].contains('i') {
                income = false;
            }
            if args[1].contains('e') {
                expenses = false;
            }
            let date;
            date = chrono::NaiveDate::parse_from_str(&args[2], "%d.%m.%Y")
                .expect("Could not parse the date make sure format is \"DD.MM.YYYY\"");
            let month = read_month_file(date)
                .unwrap_or(BudgetMonth::new(date.month0() as u8 + 1, Vec::new()));
            if !display_day {
                println!(
                    "{}",
                    month.display(date.month0() as u32 + 1, &expenses, &income, &true, &tags)
                );
                exit(0)
            }
            let day = month.get_day(date.day0() as u8 + 1);
            println!("{}", day.display(&expenses, &income, &true, &tags));
            exit(0)
        }
        let mut tags: Vec<String> = Vec::new();
        let mut income: bool = true;
        let mut expenses: bool = true;
        let mut display_day: bool = true;

        if args[1].starts_with('-') {
            if args[1].contains('m') {
                display_day = false;
            }
            if args[1].contains("d") {
                display_day = true;
            }
            if args[1].contains('t') {
                println!("Input space \" \" separated tags");
                let mut tags_string = String::new();
                io::stdin()
                    .read_line(&mut tags_string)
                    .expect("Failed to read input");
                tags = tags_string.trim().split(' ').map(str::to_string).collect();
            }
            if args[1].contains('i') {
                income = false;
            }
            if args[1].contains('e') {
                expenses = false;
            }
        }
        let date = curr_date;
        let month =
            read_month_file(date).unwrap_or(BudgetMonth::new(date.month0() as u8 + 1, Vec::new()));
        let day = month.get_day(date.day0() as u8 + 1);
        if !display_day {
            println!("{}", day.display(&expenses, &income, &true, &tags));
            exit(0)
        }
        let day = month.get_day(date.day0() as u8 + 1);
        println!("{}", day.display(&expenses, &income, &true, &tags));
        exit(0)
    }

    if args[1] == "add" {
        let mut date = curr_date;
        if args.len() == 3 {
            date = chrono::NaiveDate::parse_from_str(&args[2], "%d.%m.%Y")
                .expect("Could not parse the date make sure format is \"DD.MM.YYYY\"");
        }
        let mut month =
            read_month_file(date).unwrap_or(BudgetMonth::new(date.month0() as u8 + 1, Vec::new()));
        let mut day = month.get_day(date.day0() as u8 + 1);

        let mut name = String::new();
        let mut desc = String::new();
        let mut tags_string = String::new();
        let mut money_string = String::new();

        println!("Enter name for the entry");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");
        println!("Enter a description(optional)");
        io::stdin()
            .read_line(&mut desc)
            .expect("Failed to read input");
        println!("Enter tags(Comma seperated(exampl: shoping, luxury), optional)");
        io::stdin()
            .read_line(&mut tags_string)
            .expect("Failed to read input");
        println!("Enter the money spent(negative value) or earned(positive value) decimals should be seperated with \".\"");
        io::stdin()
            .read_line(&mut money_string)
            .expect("Failed to read input");

        let tags: Vec<String> = tags_string.trim().split(',').map(str::to_string).collect();
        let money: f32 = money_string
            .trim()
            .parse::<f32>()
            .expect("could not parse money");

        day = day.add_entrie(Entry::new(
            name.trim().to_string(),
            desc.trim().to_string(),
            tags,
            money,
        ));
        month = month.replace_day(day);
        write_month_file(&date, &month);

        exit(0);
    }

    if args[1] == "remove" {
        let mut date = curr_date;

        if args.len() == 3 {
            date = chrono::NaiveDate::parse_from_str(&args[2], "%d.%m.%Y")
                .expect("Could not parse the date make sure format is \"DD.MM.YYYY\"");
        }

        let mut month =
            read_month_file(date).unwrap_or(BudgetMonth::new(date.month0() as u8 + 1, Vec::new()));
        let mut day = month.get_day(date.day0() as u8 + 1);

        let mut text = String::new();
        let mut i = 1;
        for entry in day.get_entries() {
            text += &(i.to_string() + "\n");
            text += &entry.display();
            i += 1;
        }
        println!("Input the number of the entry you wish to remove or if multiple seperate th numbers with a space \" \" \n{}", text);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input_vec: Vec<&str> = input.trim().split(" ").collect();

        for input_string in input_vec {
            let input: usize = input_string.parse().expect("Input parse Error:");

            if input - 1 < day.get_entries().len() {
                day = day.remove_entrie(input - 1);
                continue;
            }
            println!("Input number too big");
            exit(0);
        }

        month = month.replace_day(day);
        write_month_file(&date, &month);

        exit(0);
    }

    let date;
    date = chrono::NaiveDate::parse_from_str(&args[1], "%d.%m.%Y")
        .expect("Could not parse the date make sure format is \"DD.MM.YYYY\"");
    let month =
        read_month_file(date).unwrap_or(BudgetMonth::new(date.month0() as u8 + 1, Vec::new()));
    let day = month.get_day(date.day0() as u8 + 1);
    println!("{}", day.display(&true, &true, &true, &vec![]));
    exit(0)
}
