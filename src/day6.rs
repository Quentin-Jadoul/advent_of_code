use std::collections::HashSet;

use crate::input;

pub fn day6() -> input::Result<()> {
    let contents = input::load_day_file("day6.txt");

    let window_size = 14;

    let binding = contents.chars().collect::<Vec<char>>();
    let result = binding.windows(window_size);

    for window in result {
        let mut set: HashSet<char> = HashSet::new();
        for c in window {
            set.insert(*c);
        }
        if set.len() == 14 {
            println!("Result: {}", window_size + binding.windows(14).position(|x| x == window).ok_or("no window of the good size found")?);
            break;
        }
    }
    
    Ok(())
}