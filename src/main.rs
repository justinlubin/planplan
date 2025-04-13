use chrono::prelude::*;
use std::fs;
use std::str::FromStr;

use todo_txt::Task;

fn superscript(s: &str) -> String {
    let mut ret = s.to_owned();
    for (lhs, rhs) in vec![
        ("0", "⁰"),
        ("1", "¹"),
        ("2", "²"),
        ("3", "³"),
        ("4", "⁴"),
        ("5", "⁵"),
        ("6", "⁶"),
        ("7", "⁷"),
        ("8", "⁸"),
        ("9", "⁹"),
        (")", "⁾"),
    ] {
        ret = str::replace(&ret, lhs, rhs);
    }
    ret
}

fn month_start<Tz: TimeZone>(dt: DateTime<Tz>) -> NaiveDate {
    NaiveDate::from_ymd_opt(dt.year(), dt.month(), 1).unwrap()
}

fn main() {
    let local: DateTime<Local> = Local::now();

    let tasks: Vec<Task> = fs::read_to_string("/Users/jlubin/Dropbox/notes/todo.txt")
        .unwrap()
        .lines()
        .map(|x| Task::from_str(x).unwrap())
        .collect();

    let start = month_start(local);
    let start_offset = start.weekday().num_days_from_sunday() as i64;
    println!("{}", local.format("%B"));

    let header = vec!["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

    let mut cells: Vec<Vec<Vec<String>>> = vec![];

    let mut max_height = 5;
    for row in 0..5 {
        let mut row_cells = vec![];
        for col in 0..7 {
            let mut cell = vec![];
            let offset = chrono::TimeDelta::days(row * 7 + col - start_offset);
            let dat = start + offset;
            if dat.month() != local.month() {
                row_cells.push(cell);
                continue;
            }

            cell.push(format!("{}", dat.day()));

            for (i, task) in tasks
                .iter()
                .enumerate()
                .filter(|(_, t)| t.due_date == Some(dat))
            {
                cell.push(format!(
                    "{}{}",
                    superscript(&format!("{}", i + 1)),
                    task.subject
                ));
            }

            if cell.len() > max_height {
                max_height = cell.len();
            }

            row_cells.push(cell);
        }

        cells.push(row_cells);
    }

    for h in header {
        print!("{: <19}", h)
    }
    println!("");

    for row in 0..5 {
        for line in 0..max_height {
            for col in 0..7 {
                let content = if line >= cells[row][col].len() {
                    ""
                } else {
                    &cells[row][col][line]
                };
                print!("{: <19}", content)
            }
            println!("")
        }
    }
}
