use crate::input;

pub fn day7() -> input::Result<()> {
    let content = input::load_day_file("day7.txt");

    // create a stack containing the size of the current path
    let mut stack: Vec<usize> = Vec::new();
    // create a vector containing the size of explored branches
    let mut sizes: Vec<usize> = Vec::new();

    for line in content.lines() {
        if line == "$ cd .." {
            let size = stack.pop().unwrap();
            sizes.push(size);
            let stack_len = stack.len();
            stack[stack_len - 1] += size;
        } else if line.starts_with("$ cd") {
            stack.push(0);
        } else if line.starts_with("$ ls") {
            
        } else if line.starts_with("dir ") {
            
        } else {
            let mut iter = line.split_whitespace();
            let size = iter.next().unwrap().parse::<usize>().unwrap();

            let stack_len = stack.len();
            stack[stack_len - 1] += size;
        }
    }
    while stack.len() > 1 {
        let size = stack.pop().unwrap();
        sizes.push(size);
        let stack_len = stack.len();
        stack[stack_len - 1] += size;
    }
    sizes.push(stack.pop().unwrap());


    // Step 1
    // sum all the values smaller than 100000 in the sizes vector
    // let result = sizes.iter().filter(|x| **x < 100000).sum::<usize>();
    // println!("Result: {}", result);

    // Step 2
    let free_space = 70000000 - sizes.iter().max().unwrap();
    let space_to_free = 30000000 - free_space;
    let result = sizes.iter().filter(|x| **x >= space_to_free).min().unwrap();

    println!("Result: {}", result);

    Ok(())
}