use crate:: input;
use regex::Regex;
use std::f64;

pub fn day6() -> input::Result<()> {
    let content = input::load_day_file("day6.txt");

    println!("Part 1: {}", part_1(&content));
    println!("Part 2: {}", part_2(&content));
    
    Ok(())
}

pub fn part_1(content: &String) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let lines = content.lines().collect::<Vec<_>>();
    let time: Vec<usize> = re.captures_iter(lines.get(0).unwrap())
        .map(|cap| cap[0].parse::<usize>().unwrap())
        .collect();
    let distance: Vec<usize> = re.captures_iter(lines.get(1).unwrap())
        .map(|cap| cap[0].parse::<usize>().unwrap())
        .collect();

    count_winning_cases(time, distance)
}
pub fn part_2(content: &String) -> usize {
    let time: Vec<usize> = vec![content.lines().next().unwrap().split(":").nth(1).unwrap().replace(" ", "").parse().unwrap()];

    let distance: Vec<usize> = vec![content.lines().nth(1).unwrap().split(":").nth(1).unwrap().replace(" ", "").parse().unwrap()];
    
    count_winning_cases(time, distance)
}

pub fn count_winning_cases(time: Vec<usize>, distance: Vec<usize>) -> usize {
    let mut total_winning_cases = 1;
    
    for (time, &distance) in time.iter().zip(&distance) {
        let x = ((time / 2) * (time / 2 + time % 2)) as f64 - distance as f64;
        
        if time % 2 != 0 {
            let c = -x;
            let i: usize = ((-1.0 - f64::sqrt(1.0 - 4.0*c)) / (2.0)).abs().floor() as usize;
            total_winning_cases *= 2 * i;
        } else {
            let i = (f64::sqrt(x)).ceil() as usize;
            total_winning_cases *= (2 * i) - 1;
        }
    }
    total_winning_cases
}