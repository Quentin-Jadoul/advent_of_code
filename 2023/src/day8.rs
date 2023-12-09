use crate:: input;
use regex::Regex;
use std::collections::HashMap;

use num_integer::lcm;


pub fn day8() -> input::Result<()> {
    let content = input::load_day_file("day8.txt");

    let sequence = content.lines().next().unwrap();
    // We create an empty hashmap, the key is a str and the value is a tuple of (str, str)
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    // we iterate over the other lines
    for line in content.lines().skip(1) {
        if let Some(caps) = re.captures(line) {
            let source = caps.get(1).map_or("", |m| m.as_str());
            let dest_l = caps.get(2).map_or("", |m| m.as_str());
            let dest_r = caps.get(3).map_or("", |m| m.as_str());
            map.insert(source, (dest_l, dest_r));
        }
    }

    println!("Part 1: {}", find_nb_steps(&map, sequence));
    println!("Part 2: {}", find_nb_steps_parralel(&map, sequence));
    
    Ok(())
}

pub fn find_nb_steps(map: &HashMap<&str, (&str, &str)>, sequence: &str) -> usize {
    let mut nb_steps = 0;
    let mut next_key = "AAA";
    let mut sequence_iter = sequence.chars().cycle();

    while next_key != "ZZZ" {
        let (dest_l, dest_r) = map.get(next_key).unwrap();
        let direction = sequence_iter.next().unwrap();
        next_key = if direction == 'L' { dest_l } else { dest_r };
        nb_steps += 1;
    }
    nb_steps
}

pub fn find_nb_steps_parralel(map: &HashMap<&str, (&str, &str)>, sequence: &str) -> usize {
    let mut steps_to_z = Vec::new();
    let starting_nodes: Vec<&str> = map.keys().filter(|&key| key.ends_with("A")).cloned().collect();
    let mut sequence_iter = sequence.chars().cycle();

    for node in starting_nodes {
        let mut current_node = node;
        let mut nb_steps = 0;
        while !current_node.ends_with('Z') {
            let direction = sequence_iter.next().unwrap();
            let (dest_l, dest_r) = map.get(current_node).unwrap();
            current_node = if direction == 'L' { *dest_l } else { *dest_r };
            nb_steps += 1;
        }
        steps_to_z.push(nb_steps);
    }

    steps_to_z.iter().fold(1, |acc, &x| lcm(acc, x))
}