use crate::input;

pub fn day1() -> input::Result<()> {
    let contents = input::load_day_file("day1.txt");

    let mut sum = 0;

    // create a dictionary of numbers and their corresponding words
    let dict: std::collections::HashMap<&str, usize> = 
        [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9), ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9)].iter().cloned().collect();
    // create a dictionary of a number and its position in the line, initialize it empty
    
    for line in contents.lines() {
        let mut pos: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for word in dict.keys() {
            let mut start = 0;
            while let Some(index) = line[start..].find(word) {
                pos.insert(start + index, dict[word]);
                start += index + word.len();
            }
        }
        // print the dictionary
        println!("{:?}", pos);

        // in the pos dictionary, keep only the smallest and the biggest key (position)
        let first: &usize = pos.iter().min_by_key(|x| x.0).unwrap().1;
        let last = pos.iter().max_by_key(|x| x.0).unwrap().1;

        // concatenate the first and last number
        let num = format!("{}{}", first, last).parse::<usize>().unwrap();
        println!("{}", num);

        sum += num;
    }

    println!("Part 1: {}", sum);
    
    Ok(())
}

