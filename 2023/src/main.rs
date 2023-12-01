use std::env;

mod day1;
// mod day18;

mod input;

use input::Day;

fn main() {
    let args: Vec<String> = env::args().collect();

    let days = vec![
        Day::new("Day 1", day1::day1),
    ];
    for day in args.iter().skip(1) {
        match day.as_str() {
            "day1" => days[0].run(),
            _ => println!("Day not found"),
        }
    }

    if args.len() == 1 {
        for day in days {
            day.run();
        }
    }
}
