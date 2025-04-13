use chrono::prelude::*;
use std::fs;
use std::io::Write;
use std::process::Command;
use std::str::FromStr;

use todo_txt::Task;

// https://stackoverflow.com/a/38461750
fn truncate_pretty(s: &str, max_chars: usize) -> String {
    match s.char_indices().nth(max_chars) {
        None => s.to_owned(),
        Some((idx, _)) => format!("{}..", &s[..idx - 2]),
    }
}

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

fn month_sugar(dt: DateTime<Local>, s: &str) -> String {
    match s.parse::<i32>() {
        Ok(x) => format!("{}{}", dt.format("%Y-%m-"), x),
        Err(_) => s.to_owned(),
    }
}

fn main() {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        let local: DateTime<Local> = Local::now();

        let tasks: Vec<Task> = fs::read_to_string("/Users/jlubin/Dropbox/notes/todo.txt")
            .unwrap()
            .lines()
            .map(|x| Task::from_str(x).unwrap())
            .collect();

        let start = month_start(local);
        let start_offset = start.weekday().num_days_from_sunday() as i64;
        println!("|{:=^131}|", local.format(" %B "));

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
            // print!("{:-<18} ", format!("{} ", h))
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
                    print!("{: <19}", truncate_pretty(content, 19))
                }
                println!("")
            }
        }

        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "q" || input == "quit" {
            return;
        }

        let mut it = input.splitn(2, " ");
        let cmd = match it.next() {
            Some(x) => x,
            None => continue,
        };
        let rest = match it.next() {
            Some(x) => x,
            None => continue,
        };

        if cmd == "a" || cmd == "add" {
            let mut it = rest.splitn(2, " ");
            let due = month_sugar(
                local,
                match it.next() {
                    Some(x) => x,
                    None => continue,
                },
            );
            let subject = match it.next() {
                Some(x) => x,
                None => continue,
            };

            Command::new("todo.sh")
                .args(["add", &format!("{} due:{}", subject, due)])
                .output()
                .unwrap();
        } else if cmd == "d" || cmd == "do" {
            Command::new("todo.sh").args(["do", rest]).output().unwrap();
        } else if cmd == "m" || cmd == "move" {
        }
    }
}
