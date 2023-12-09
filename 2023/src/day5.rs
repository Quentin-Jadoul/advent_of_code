use crate:: input;
use std::collections::HashMap;

pub fn day5() -> input::Result<()> {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
    
    Ok(())
}

pub fn part_1() -> usize {
    let content = input::load_day_file("day5.txt");

    // Split the content on empty lines
    let groups: Vec<&str> = content.split("\n\n").collect();

    // Get the first group and parse it into seeds
    let seeds: Vec<usize> = groups[0].split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    // Parse the rest of the groups into maps
    let maps: Vec<HashMap<usize, (usize, usize)>> = groups[1..].iter().map(|group| {
        group.lines().filter_map(|line| {
            let nums: Vec<usize> = line.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            if nums.len() == 3 {
                Some((nums[0], (nums[1], nums[2])))
            } else {
                None
            }
        }).collect()
    }).collect();

    // Call the find_min_location function with the seeds and maps
    find_min_location(&seeds, &maps)
}

pub fn part_2() -> usize {
    0
}

fn transform_number(number: usize, map: &HashMap<usize, (usize, usize)>) -> usize {
    for (&dest_range_start, &(source_range_start, range_length)) in map {
        if number >= source_range_start && number < source_range_start + range_length {
            return number + dest_range_start - source_range_start;
        }
    }
    number
}

fn find_min_location(seeds: &[usize], maps: &[HashMap<usize, (usize, usize)>]) -> usize {
    seeds.iter().map(|&seed| {
        maps.iter().fold(seed, |number, map| transform_number(number, map))
    }).min().unwrap()
}