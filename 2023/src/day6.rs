use crate:: input;
use regex::Regex;

pub fn day6() -> input::Result<()> {
    let content = input::load_day_file("day6.txt");

    println!("Part 1: {}", part_1(content.clone()));
    println!("Part 2: {}", part_2(content.clone()));
    
    Ok(())
}

pub fn part_1(content: String) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let mut time = Vec::new();
    for cap in re.captures_iter(&content.lines().next().unwrap()) {
        time.push(cap[0].parse::<usize>().unwrap());
    }
    let mut distance = Vec::new();
    for cap in re.captures_iter(&content.lines().nth(1).unwrap()) {
        distance.push(cap[0].parse::<usize>().unwrap());
    }

    count_winning_cases(time, distance)
}
pub fn part_2(content: String) -> usize {
    let mut time = Vec::new();
    time.push(content.lines().next().unwrap().split(":").nth(1).unwrap().replace(" ", "").parse().unwrap());

    let mut distance = Vec::new();
    distance.push(content.lines().nth(1).unwrap().split(":").nth(1).unwrap().replace(" ", "").parse().unwrap());
    
    count_winning_cases(time, distance)
}

pub fn count_winning_cases(time: Vec<usize>, distance: Vec<usize>) -> usize {
    let mut total_winning_cases = 1;
    for (index, time) in time.iter().enumerate() {
        let part1 = time / 2;
        let part2 = time / 2 + time % 2;
        // if time is even
        let mut winning_cases = 0;
        let mut i = 0;
        if time % 2 != 0 {
            while (part1 * part2) - (i * (i+1)) > distance[index] {
                winning_cases = 2 * (i + 1);
                i += 1;
            } 
        } else {
            while (part1 * part2) - (i * i) > distance[index] {
                winning_cases = 1 + (2 * i);
                i += 1;
            }
            
        }
        total_winning_cases *= winning_cases;
    }
    total_winning_cases
}