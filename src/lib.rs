use chrono::{Datelike, Local, NaiveDate};
use std::io::{Result, Write};
use  std::path::PathBuf;
use std::fs::{self, *};
use serde::{Deserialize, Serialize};

fn get_default_path() -> PathBuf {
    home::home_dir().unwrap().join(PathBuf::from("Documents/Budget_Manager/"))
}

////////////////
/////TESTS//////
////////////////
#[test]
fn day_total() {
    let entries = vec![
        Entry::new("".to_string(), "".to_string(), vec![], -1.0),
        Entry::new("".to_string(), "".to_string(), vec!["test".to_string()], -2.0),
        Entry::new("".to_string(), "".to_string(), vec![], 3.0),
        Entry::new(
            "".to_string(),
            "".to_string(),
            vec!["test".to_string(), "test2".to_string()],
            4.0,
        ),
    ];
    let day = BudgetDay::new(1, entries);
    let day2 = BudgetDay::new(1, Vec::new());

    assert_eq!(day.total(&false, &Vec::new()), -3.0);
    assert_eq!(day.total(&true, &Vec::new()), 4.0);
    assert_eq!(day.total(&false, &vec!["test".to_string()]), -2.0);
    assert_eq!(day.total(&true, &vec!["test".to_string()]), 2.0);
    assert_eq!(
        day.total(&true, &vec!["test".to_string(), "test2".to_string()]),
        4.0
    );
    assert_eq!(day2.total(&false, &Vec::new()), 0.0);
}

#[test]
fn day_average() {
    let entries = vec![
        Entry::new("".to_string(), "".to_string(), vec![], -1.0),
        Entry::new("".to_string(), "".to_string(), vec!["test".to_string()], -2.0),
        Entry::new("".to_string(), "".to_string(), vec![], 3.0),
        Entry::new(
            "".to_string(),
            "".to_string(),
            vec!["test".to_string(), "test2".to_string()],
            4.0,
        ),
    ];
    let day = BudgetDay::new(1, entries);
    let day2 = BudgetDay::new(1, Vec::new());

    assert_eq!(day.average(&false, &Vec::new()), (-1.5, 2));
    assert_eq!(day.average(&true, &Vec::new()), (1.0, 4));
    assert_eq!(day.average(&false, &vec!["test".to_string()]), (-2.0, 1));
    assert_eq!(day.average(&true, &vec!["test".to_string()]), (1.0, 2));
    assert_eq!(
        day.average(&true, &vec!["test".to_string(), "test2".to_string()]),
        (4.0, 1)
    );
    assert_eq!(day2.average(&false, &Vec::new()), (0.0, 0));
}

#[test]
fn day_print() {
    let entries = vec![
        Entry::new("1".to_string(), "desc".to_string(), vec![], -1.0),
        Entry::new(
            "2".to_string(),
            "desc".to_string(),
            vec!["".to_string()],
            -2.0,
        ),
        Entry::new("3".to_string(), "desc".to_string(), vec![], 3.0),
        Entry::new(
            "4".to_string(),
            "desc".to_string(),
            vec!["test".to_string(), "test2".to_string()],
            4.0,
        ),
    ];
    let day = BudgetDay::new(1, entries);
    let day2 = BudgetDay::new(1, Vec::new());

    print!("{}", day.display(&true, &true, &Vec::new()));
    print!("{}", day.display(&false, &true, &Vec::new()));
    print!("{}", day.display(&true, &true, &vec!["test".to_string()]));
    print!("{}", day.display(&false, &true, &vec!["test".to_string()]));
    print!(
        "{}",
        day.display(&true, &true, &vec!["test".to_string(), "test2".to_string()])
    );
    print!("{}", day2.display(&true, &true, &Vec::new()));
}

#[test]
fn month_average() {
    let values_1 = vec![9.0, -5.0, 2.0];
    let values_2 = vec![-2.0, -1.0, 6.0];
    let mut entries_1: Vec<Entry> = vec![];
    let mut entries_2: Vec<Entry> = vec![];
    let total_entries = (&values_1.len() + &values_2.len()) as u32;
    let mut average = 0.0;

    for value in &values_1 {
        entries_1.push(Entry::new("".to_string(), "".to_string(), vec![], *value));
        average += value;
    }
    for value in &values_2 {
        entries_2.push(Entry::new("".to_string(), "".to_string(), vec![], *value));
        average += value;
    }

    let days = vec![
        BudgetDay::new(1, entries_1),
        BudgetDay::new(2, entries_2),
    ];

    average =  average / total_entries as f32;

    let month = BudgetMonth::new(1, days);

    assert_eq!(month.average(&true, &vec![]), (average, total_entries));
}

#[test]
fn month_display() {
    
    let entries_1 = vec![
        Entry::new("1".to_string(), "desc".to_string(), vec![], -1.0),
        Entry::new(
            "2".to_string(),
            "desc".to_string(),
            vec!["".to_string()],
            -2.0,
        ),
    ];
    let entries_2 = vec![
        Entry::new("3".to_string(), "desc".to_string(), vec![], 3.0),
        Entry::new(
            "4".to_string(),
            "desc".to_string(),
            vec!["test".to_string(), "test2".to_string()],
            4.0,
        ),
    ];
    let days = vec![
    BudgetDay::new(1, entries_1),
    BudgetDay::new(2, entries_2),
    BudgetDay::new(1, Vec::new())
    ];
    
    let month = BudgetMonth::new(1, days);

    print!("{}", month.display(&true, &true, &Vec::new()));
    print!("{}", month.display(&false, &true, &Vec::new()));
    print!("{}", month.display(&true, &true, &vec!["test".to_string()]));
    print!("{}", month.display(&false, &true, &vec!["test".to_string()]));
    print!(
        "{}",
        month.display(&true, &true, &vec!["test".to_string(), "test2".to_string()])
    );


}

    #[test]
    fn file_creation() {
    let entries_1 = vec![
        Entry::new("1".to_string(), "desc".to_string(), vec![], -1.0),
        Entry::new(
            "2".to_string(),
            "desc".to_string(),
            vec!["".to_string()],
            -2.0,
        ),
    ];
    let entries_2 = vec![
        Entry::new("3".to_string(), "desc".to_string(), vec![], 3.0),
        Entry::new(
            "4".to_string(),
            "desc".to_string(),
            vec!["test".to_string(), "test2".to_string()],
            4.0,
        ),
    ];
    let days = vec![
    BudgetDay::new(1, entries_1),
    BudgetDay::new(2, entries_2),
    BudgetDay::new(1, Vec::new())
    ];
    
    let month = BudgetMonth::new(1, days);

        let curr_date = Local::now().date_naive();
        check_files(&curr_date);
        write_month_file(&curr_date, &month);
        
    }
////////////////
//////CODE//////
////////////////
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    name: String,
    desc: String,
    tags: Vec<String>,
    money: f32,
}

impl Entry {
    // add code here
    pub fn new(name: String, desc: String, tags: Vec<String>, money: f32) -> Entry {
        Entry {
            name,
            desc,
            tags,
            money,
        }
    }
    pub fn display(&self) -> String {
        let mut tags = String::new();
        for tag in &self.tags {
            tags += &(tag.to_owned() + ", ");
        }
        format!(
            "
            Name: {}
            Tags: {}
            Description: {}

            ${}

            ",
            self.name, tags, self.desc, self.money,
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BudgetDay {
    day_of_month: u8,
    entries: Vec<Entry>,
}

impl BudgetDay {
    pub fn new(day_of_month: u8, entries: Vec<Entry>) -> BudgetDay {
        BudgetDay {
            day_of_month,
            entries,
        }
    }

    pub fn add_entrie(mut self, entrie: Entry) -> BudgetDay {
        self.entries.push(entrie);
        self
    }

    pub fn total(&self, with_income: &bool, tags: &Vec<String>) -> f32 {
        if self.entries.is_empty() {
            return 0.0;
        }

        let mut total = 0.0;

        'main: for entrie in &self.entries {
            if !with_income && entrie.money > 0.0 {
                continue;
            }
            for tag in tags {
                if !entrie.tags.contains(tag) {
                    continue 'main;
                }
            }
            total += entrie.money as f32;
        }

        total
    }

    pub fn average(&self, with_income: &bool, tags: &Vec<String>) -> (f32, u32) {
        if self.entries.is_empty() {
            return (0.0, 0);
        }

        let mut total_money = 0.0;
        let mut total_entries = 0;

        'main: for entry in &self.entries {
            if !with_income && entry.money > 0.0 {
                continue;
            }
            for tag in tags {
                if !entry.tags.contains(tag) {
                    continue 'main;
                }
            }
            total_entries += 1;
            total_money += entry.money;
        }

        if total_entries == 0 {
            return (0.0, 0);
        }

        let average = total_money / total_entries as f32;
        (average, total_entries)
    }

    pub fn display(&self, with_income: &bool, with_entries: &bool, tags: &Vec<String>) -> String {
        let mut entries = String::new();
        let mut total_entries = 0;

        'main: for entry in &self.entries {
            if !with_income && entry.money > 0.0 {
                continue;
            }
            for tag in tags {
                if !entry.tags.contains(tag) {
                    continue 'main;
                }
            }
            total_entries += 1;
            if *with_entries {
                entries += &entry.display();
            }
        }
        format! {"
################
Day of month: {}
################

    {}

Daily Stats:

Total entries: {}

Total: {}

Average: {}
            ",
            self.day_of_month,
            entries,
            total_entries,
            self.total(with_income, tags),
            self.average(with_income, tags).0
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BudgetMonth {
    month_of_year: u8,
    days: Vec<BudgetDay>,
}

impl BudgetMonth {
    pub fn new(month_of_year: u8, days: Vec<BudgetDay>) -> BudgetMonth {
        BudgetMonth {
            month_of_year,
            days,
        }
    }

    pub fn add_day(mut self, day: BudgetDay) -> BudgetMonth {
        self.days.push(day);
        self
    }

    pub fn get_day(&self, num_day: u8) -> BudgetDay{
        for day in &self.days {
            if day.day_of_month == num_day {
                return day.clone();
            }
        }
        BudgetDay::new(num_day, vec![])
    }

    pub fn total(&self, with_income: &bool, tags: &Vec<String>) -> f32 {
        if self.days.is_empty() {
            return 0.0;
        }

        let mut total = 0.0;

        for day in &self.days {
            total += day.total(with_income, tags);
        }

        total
    }

    pub fn average(&self, with_income: &bool, tags: &Vec<String>) -> (f32, u32) {
        if self.days.is_empty() {
            return (0.0, 0);
        }

        let mut average = 0.0;
        let mut total_entries = 0;

        for day in &self.days {
            let day_average = day.average(with_income, tags);
            total_entries += day_average.1;
            average += day_average.0 * day_average.1 as f32;
        }

        if total_entries == 0 {
            return (0.0, 0);
        }

        average = average / total_entries as f32;


        (average, total_entries)
    }

    pub fn display(&self, year: u8, with_income: &bool, with_entries: &bool, tags: &Vec<String>) -> String {

        let mut days_string = String::new();

        if *with_entries {

            let mut days_vector: Vec<BudgetDay> = vec![];

            'main: for day_num in 1..=get_days_from_month(year as i32, self.month_of_year as u32) {
                for day in &self.days {
                    if day.day_of_month as i64 == day_num{
                        days_vector.push(day.clone());
                        continue 'main;
                    }
                }
                days_vector.push(BudgetDay::new(day_num as u8, vec![]));

            }
            for day in days_vector{
                days_string += &day.display(with_income, &false, tags);
            }

            days_string.push_str("
\n###################"
            );
        }

        let mut total = 0.0;
        for day in &self.days {
            total += day.total(with_income, tags);
        }

        let average_day = total / get_days_from_month(year as i32, self.month_of_year as u32) as f32;

        let average_entrie = self.average(with_income, tags);

        format! {"
###################
#Month of Year: {}#
###################

{}

Monthly Stats:

Total entries: {}

Total: {}

Average per entrie: {}

Average per day: {}
            ",
            self.month_of_year,
            days_string,
            average_entrie.1,
            self.total(with_income, tags),
            average_entrie.0,
            average_day
        }
    }
}

 pub fn get_days_from_month(year: i32, month: u32) -> i64{
    NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .expect("")
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).expect(""))
    .num_days()
}

pub fn check_files(date: &NaiveDate){

    let path = get_default_path();
    let year_path = path.join(date.year().to_string());
    let month_path = path.join(date.year().to_string() + "/" + &(date.month0() + 1).to_string());

    if !path.exists(){
        create_dir(path).expect("Cannot create main folder");
    }

    if !year_path.exists() {
        create_dir(year_path).expect("Cannot create year folder");
    }

    if !month_path.exists() {
        File::create(month_path).expect("Cannot create month file");
    }

}

pub fn write_month_file (date: &NaiveDate, b_month: &BudgetMonth) {
    check_files(date);
    let file_path= get_default_path().join(date.year().to_string() + "/" + &(date.month0() + 1).to_string());
        let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();
    let contents = serde_json::to_string(b_month).unwrap();
    let _ = file.write_all(contents.as_bytes());

}

pub fn read_month_file (date: NaiveDate) -> Result<BudgetMonth>{

    let file_path= get_default_path().join(date.year().to_string() + "/" + &(date.month0() + 1).to_string());

    let result: BudgetMonth = serde_json::from_str(&fs::read_to_string(file_path)?)?;
    Ok(result)
}
