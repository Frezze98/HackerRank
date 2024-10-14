use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn timeConversion(s: &str) -> String {
    let (time, period) = s.split_at(8);
    let hours: i32 = time[0..2].parse().unwrap();
    let mut converted_hours = hours;

    if period == "PM" && hours != 12 {
        converted_hours += 12;
    } else if period == "AM" && hours == 12 {
        converted_hours = 0;
    }

    format!("{:02}{}", converted_hours, &time[2..])
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
