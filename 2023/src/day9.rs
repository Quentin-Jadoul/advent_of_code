use crate:: input;


pub fn day9() -> input::Result<()> {
    let content = input::load_day_file("day9.txt");

    println!("Part 1: {}", part_1(&content));
    println!("Part 2: {}", part_2(&content));
    
    Ok(())
}

pub fn part_1(content: &str) -> isize {
    content.lines().map(|line| {
        let sequence = parse_sequence(line);
        next_in_sequence(&sequence)
    }).sum()
}

pub fn part_2(content: &str) -> usize {
    content.lines().map(|line| {
        let sequence = parse_sequence(line);
        prev_in_sequence(&sequence)
    }).sum::<isize>() as usize
}

pub fn parse_sequence(line: &str) -> Vec<isize> {
    line.split_whitespace()
        .filter_map(|s| s.parse::<isize>().ok())
        .collect()
}

pub fn next_in_sequence(sequence: &[isize]) -> isize {
    let diffs: Vec<_> = sequence.windows(2).map(|w| w[1] - w[0]).collect();
    if diffs.iter().all(|&diff| diff == 0) {
        *sequence.last().unwrap()
    } else {
        let next_term = next_in_sequence(&diffs);
        sequence.last().unwrap() + next_term
    }
}

pub fn prev_in_sequence(sequence: &[isize]) -> isize {
    let diffs: Vec<_> = sequence.windows(2).map(|w| w[1] - w[0]).collect();
    if diffs.iter().all(|&diff| diff == 0) {
        *sequence.first().unwrap() - diffs[0]
    } else {
        let prev_term = prev_in_sequence(&diffs);
        sequence.first().unwrap() - prev_term
    }
}