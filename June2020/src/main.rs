#[macro_use] extern crate text_io;

mod day1; mod day3; mod day4; mod day5; mod day6;

fn main() {
    println!("Select day to run from June challenge: ");
    let line: String = read!("{}\n");
    let day = line.parse::<i32>().expect("Failed to parse day!");

    match day {
        1 => day1::run(),
        2 => println!("Day 2 not available in Rust, see day2.py."),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        v => println!("Day {} challenge not available!", v),
    }
}
