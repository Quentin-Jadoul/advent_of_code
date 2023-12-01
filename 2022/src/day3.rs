use crate::input;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn day3() -> input::Result<()> {
    let contents = input::load_day_file("day3.txt");

    let mut priorities_sum: u32 = 0;

    let categories: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let mut priorities: HashMap<char, u32> = HashMap::new();
    for (i, c) in categories.iter().enumerate() {
        priorities.insert(*c, (i + 1) as u32);
    }

    for rucksack in contents.lines() {
        let (left, right) = rucksack.split_at(rucksack.len() / 2);
        let set: HashSet<char> = left.chars().collect();

        right.chars().any(|c| {
            if set.contains(&c) {
                priorities_sum += priorities.get(&c).unwrap();
                true
            } else {
                false
            }
        });
    }
    println!("Step 1: {}", priorities_sum);

    // Step 2
    let mut badge_priority = 0;

    let mut groups: Vec<Vec<&str>> = Vec::new();
    let mut group: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if group.len() == 3 {
            groups.push(group);
            group = Vec::new();
        }
        group.push(line);
    }
    // add last group
    groups.push(group);

    for group in groups {
        let mut set: HashSet<char> = group[0].chars().collect();
        for rucksack in group.iter().skip(1) {
            set = set.intersection(&rucksack.chars().collect()).map(|x| *x).collect();
            if set.len() == 1 {
                badge_priority += priorities.get(set.iter().next().unwrap()).unwrap();
            }
        }
    }

    println!("Step 2: {}", badge_priority);
    
    Ok(())
}