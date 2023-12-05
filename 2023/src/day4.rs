use regex::Regex;
use crate:: input;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn day4() -> input::Result<()> {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
    
    Ok(())
}

pub fn part_1() -> usize {
    let content = input::load_day_file("day4.txt");

    let mut sum = 0;

    for line in content.lines() {
        let (_id, winning_numbers, numbers) = parse(line);

        let mut points = 0;

        for number in numbers.iter() {
            if winning_numbers.contains(number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        sum += points;
    }

    sum
}
pub fn part_2() -> usize {
    let content = input::load_day_file("day4.txt");

    let mut sum = 0;

    // Create a hashmap with the id of the card as key and the number of copy as value
    let mut cards: HashMap<usize, usize> = HashMap::new();
    cards.insert(1, 1);

    for line in content.lines() {
        let (id, winning_numbers, numbers) = parse(line);
    
        let mut i = 0;
    
        let value = *cards.entry(id).or_insert(1);
        sum += value;
        for _ in winning_numbers.intersection(&numbers) {
            i += 1;
            let copy_id = id + i;
            *cards.entry(copy_id).or_insert(1) += value;
        }
    }
    sum
}

pub fn parse(line: &str) -> (usize, HashSet<&str>, HashSet<&str>) {
    // split on the : and | characters
    let (id, card_content) = line.split_once(":").unwrap();
    let (winning_numbers, numbers) = card_content.split_once("|").unwrap();

    // convert id to usize and keep only the number
    let id = id.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();

    let re = Regex::new(r"\b\d+\b").unwrap();
    let winning_numbers: HashSet<&str> = re.captures_iter(winning_numbers)
        .map(|cap| cap.get(0).map_or("", |m| m.as_str()))
        .collect();
    let numbers: HashSet<&str> = re.captures_iter(numbers)
        .map(|cap| cap.get(0).map_or("", |m| m.as_str()))
        .collect();

    (id, winning_numbers, numbers)
}