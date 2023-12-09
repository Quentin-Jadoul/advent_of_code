use crate:: input;
use regex::Regex;
use std::f64;

pub fn day6() -> input::Result<()> {
    let content = include_str!("../data/day6.txt");

    println!("Part 1: {}", part_1(content));
    println!("Part 2: {}", part_2(content));
    
    Ok(())
}

pub fn part_1(content: &str) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let lines = content.lines().collect::<Vec<_>>();
    let time: Vec<usize> = re.captures_iter(lines.get(0).unwrap())
        .map(|cap| cap[0].parse::<usize>().unwrap())
        .collect();
    let distance: Vec<usize> = re.captures_iter(lines.get(1).unwrap())
        .map(|cap| cap[0].parse::<usize>().unwrap())
        .collect();

    count_winning_cases(&time, &distance)
}
pub fn part_2(content: &str) -> usize {
    let time: Vec<usize> = vec![content.lines().next().unwrap().split(":").nth(1).unwrap().replace(" ", "").parse().unwrap()];

    let distance: Vec<usize> = vec![content.lines().nth(1).unwrap().split(":").nth(1).unwrap().replace(" ", "").parse().unwrap()];
    
    count_winning_cases(&time, &distance)
}

pub fn count_winning_cases(time: &[usize], distance: &[usize]) -> usize {
    time.iter().zip(distance).fold(1, |acc, (&time, &distance)| {
        let x = ((time / 2) * (time / 2 + time % 2)) as f64 - distance as f64;
        if time % 2 != 0 {
            let c = -x;
            let i: usize = ((-1.0 - (1.0 - 4.0*c).sqrt()) / (2.0)).abs().floor() as usize;
            acc * (2 * i)
        } else {
            let i = ((x).sqrt()).ceil() as usize;
            acc * ((2 * i) - 1)
        }
    })
}