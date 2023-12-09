use crate::input;
use std::str::FromStr;
use std::collections::BTreeMap;

pub fn day7() -> input::Result<()> {
    let content = input::load_day_file("day7.txt");
    println!("Part 1: {}", process_hands(&content, false));
    println!("Part 2: {}", process_hands(&content, true));
    
    Ok(())
}

pub fn process_hands(content: &String, is_part2: bool) -> usize {
    let mut hands: Vec<(usize, usize)> = Vec::new();

    for line in content.lines() {
        let mut words = line.split_whitespace();
        let hand_string = words.next().unwrap();
        let mut counts: BTreeMap<char, usize> = BTreeMap::new();
    
        let hand: String = hand_string.chars().map(|c| {
            let converted_char = match c {
                'T' => 'A',
                'J' => if is_part2 { '1' } else { 'B' },
                'Q' => 'C',
                'K' => 'D',
                'A' => 'E',
                _ => c,
            };
            *counts.entry(converted_char).or_insert(0) += 1;
            converted_char
        }).collect();
    
        if counts.len() > 1 && is_part2 {
            if let Some(val) = counts.remove(&'1') {
                if let Some(max_value) = counts.values_mut().max() {
                    *max_value += val;
                }
            }
        }
        check_type(hand, counts, words.next().unwrap(), &mut hands);
    }
    
    hands.sort_by_key(|a| a.0);
    hands.iter().enumerate().map(|(i, (_, b))| (i + 1) * b).sum()
}

pub fn check_type(hand: String, counts: BTreeMap<char, usize>, word: &str, hands: &mut Vec<(usize, usize)>) {
    let mut frequencies: Vec<_> = counts.values().cloned().collect();
    frequencies.sort_unstable();

    let hand = match frequencies.as_slice() {
        [1, 1, 1, 2,] => hand + "0",
        [1, 2, 2,] => hand + "00",
        [1, 1, 3,] => hand + "000",
        [2, 3,] => hand + "0000",
        [1, 4,] => hand + "00000",
        [5] => hand + "000000",
        _ => hand,
    };

    hands.push((usize::from_str_radix(&hand, 16).unwrap(), usize::from_str(word).unwrap()));
}