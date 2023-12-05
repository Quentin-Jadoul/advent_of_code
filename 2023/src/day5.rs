use crate:: input;

pub fn day5() -> input::Result<()> {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
    
    Ok(())
}

pub fn part_1() -> usize {
    let content = input::load_day_file("day5.txt");

    // Split the content on empty lines
    let groups: Vec<&str> = content.split("\n\n").collect();

    // Get the first group
    let seeds: Vec<usize> = groups[0].split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    // For each group, get the group and the index
    let mut location: Vec<usize> = Vec::new();
    for seed in seeds.iter() {
        let mut source = *seed;
        for (i, group) in groups.iter().enumerate().skip(1) {
            for line in group.lines().skip(1) {
                let numbers: Vec<usize> = line.split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                let (dest_range_start, source_range_start, range_length) = (numbers[0], numbers[1], numbers[2]);
                if (&source >= &source_range_start) && (&source <= &(source_range_start + range_length)) {
                    let offset: isize = dest_range_start as isize - source_range_start as isize;
                    source = (source as isize + offset) as usize;
                    break;
                }
            }
        }
        location.push(source as usize);
    }
    // Get the min value
    location.iter().min().unwrap().clone()
}
pub fn part_2() -> usize {
    0
}