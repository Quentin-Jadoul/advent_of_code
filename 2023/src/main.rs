use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

mod input;

use input::Day;

fn main() {
    let args: Vec<String> = env::args().collect();

    let days = vec![
        Day::new("Day 1", day1::day1),
        Day::new("Day 2", day2::day2),
        Day::new("Day 3", day3::day3),
        Day::new("Day 4", day4::day4),
        Day::new("Day 5", day5::day5),
    ];
    for day in args.iter().skip(1) {
        match day.as_str() {
            "day1" => days[0].run(),
            "day2" => days[1].run(),
            "day3" => days[2].run(),
            "day4" => days[3].run(),
            "day5" => days[4].run(),
            _ => println!("Day not found"),
        }
    }

    if args.len() == 1 {
        for day in days {
            day.run();
        }
    }
}
