use crate::input;

pub fn day1() -> input::Result<()> {
    let contents = input::load_day_file("day1.txt");

    // Read the file and split on empty lines
    let mut groups: Vec<Vec<u32>> = Vec::new();
    let mut group: Vec<u32> = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            groups.push(group);
            group = Vec::new();
        } else {
            // push the line to the group
            group.push(line.parse::<u32>().unwrap());   
        }
    }
    // add last group
    groups.push(group);

    // Step 1
    let m = groups.iter().map(|x| x.iter().sum::<u32>()).max().ok_or("No max found")?;
    println!("Step 1: {}", m);
    
    // step 2
    let mut sums: Vec<u32> = groups.iter().map(|x| x.iter().sum::<u32>()).collect();
    sums.sort();
    let top_three: u32 = sums.iter().rev().take(3).sum();
    println!("Step 2: {:?}", top_three);
    
    Ok(())
}