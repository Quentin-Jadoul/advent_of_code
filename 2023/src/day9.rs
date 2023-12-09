use crate:: input;

pub fn day9() -> input::Result<()> {
    let content = input::load_day_file("day9.txt");
    
    // Sum each extremity of the sequence
    let (part_1_sum, part_2_sum) = content.lines().map(|line| {
        let sequence = parse_sequence(line);
        let (extreme_1, extreme_2) = sequence_extremes(&sequence);
        (extreme_1, extreme_2)
    }).fold((0, 0), |(part_1_acc, part_2_acc), (part_1, part_2)| (part_1_acc + part_1, part_2_acc + part_2));
    
    println!("Part 1: {}", part_1_sum);
    println!("Part 2: {}", part_2_sum);

    Ok(())
}

pub fn parse_sequence(line: &str) -> Vec<isize> {
    line.split_whitespace()
        .filter_map(|s| s.parse::<isize>().ok())
        .collect()
}

pub fn sequence_extremes(sequence: &[isize]) -> (isize, isize) {
    let diffs: Vec<_> = sequence.windows(2).map(|w| w[1] - w[0]).collect();
    if diffs.iter().all(|&diff| diff == 0) {
        (*sequence.last().unwrap() + diffs[0], *sequence.first().unwrap() - diffs[0])
    } else {
        let (next_term, prev_term) = sequence_extremes(&diffs);
        (*sequence.last().unwrap() + next_term, *sequence.first().unwrap() - prev_term)
    }
}
