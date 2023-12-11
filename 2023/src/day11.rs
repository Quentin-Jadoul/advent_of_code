use std::{collections::HashSet, cmp::{min, max}};
use crate:: input;

pub fn day11() -> input::Result<()> {
    let content: String = input::load_day_file("day11.txt");
    let coordinates: HashSet<(usize, usize)> = parse_input(&content);

    let (part_1, part_2) = calculate_distance(coordinates, content);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    
    Ok(())
}

pub fn parse_input(content: &str) -> HashSet<(usize,usize)> {
    content.lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(j, c)| if c == '#' { Some((j, i)) } else { None })
        })
        .collect()
}

pub fn calculate_distance(coordinates: HashSet<(usize, usize)>, content: String) -> (usize, usize) {
    let max_row = content.lines().count();
    let max_col = content.lines().next().unwrap_or("").len();

    let empty_rows: Vec<_> = (0..max_row).filter(|i| !coordinates.iter().any(|&(_, y)| y == *i)).collect();
    let empty_cols: Vec<_> = (0..max_col).filter(|j| !coordinates.iter().any(|&(x, _)| x == *j)).collect();

    let (mut sum_1, mut sum_2) = (0, 0);
    for (i, a) in coordinates.iter().enumerate() {
        for b in coordinates.iter().skip(i + 1) {
            let base_distance = (max(a.0, b.0) - min(a.0, b.0)) + (max(a.1, b.1) - min(a.1, b.1));
            let empty_row_count = empty_rows.iter().filter(|&&i| i > min(a.1, b.1) && i < max(a.1, b.1)).count();
            let empty_col_count = empty_cols.iter().filter(|&&j| j > min(a.0, b.0) && j < max(a.0, b.0)).count();

            sum_1 += base_distance + empty_row_count + empty_col_count;
            sum_2 += base_distance + (1000000 - 1) * (empty_row_count + empty_col_count);
        }
    }
    (sum_1, sum_2)
}
