use regex::Regex;
use crate:: input;

pub fn day4() -> input::Result<()> {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
    
    Ok(())
}

pub fn part_1() -> usize {
    let content = input::load_day_file("day4.txt");

    let mut sum = 0;

    for line in content.lines() {
        // split on the : and | characters
        let card_content = line.split(":").nth(1).unwrap();
        let (winning_numbers, numbers) = card_content.split_once("|").unwrap();

        let re = Regex::new(r"\b\d+\b").unwrap();
        let winning_numbers: Vec<&str> = re.captures_iter(winning_numbers)
            .map(|cap| cap.get(0).map_or("", |m| m.as_str()))
            .collect();
        let numbers: Vec<&str> = re.captures_iter(numbers)
            .map(|cap| cap.get(0).map_or("", |m| m.as_str()))
            .collect();

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
    let mut cards: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    cards.insert(1, 1);

    for line in content.lines() {
        // split on the : and | characters
        let (id, card_content) = line.split_once(":").unwrap();
        let (winning_numbers, numbers) = card_content.split_once("|").unwrap();

        // convert id to usize and keep only the number
        let id = id.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();

        let re = Regex::new(r"\b\d+\b").unwrap();
        let winning_numbers: Vec<&str> = re.captures_iter(winning_numbers)
            .map(|cap| cap.get(0).map_or("", |m| m.as_str()))
            .collect();
        let numbers: Vec<&str> = re.captures_iter(numbers)
            .map(|cap| cap.get(0).map_or("", |m| m.as_str()))
            .collect();

        let mut i = 0;

        if !cards.contains_key(&id) {
            cards.insert(id, 1);
        }
        sum += cards.get(&id).unwrap().clone();
        for number in numbers.iter() {
            if winning_numbers.contains(number) {
                i += 1;
                let copy_id = id + i;
                let multiplicator:usize = cards.get(&id).unwrap().clone();
                if cards.contains_key(&copy_id) {
                    // get the number of copies
                
                    cards.insert(copy_id, cards.get(&copy_id).unwrap() + multiplicator);
                } else {
                    cards.insert(copy_id, 1 + multiplicator);
                }
            }
        }

        
    }

    sum
}

pub fn parse() {

}