
use crate::input;

//use regex
use regex::Regex;

pub fn day5() -> input::Result<()> {
    let contents = input::load_day_file("day5.txt");

    let (start_positions,instructions) = contents.split_once("\n.").ok_or("expected input separated by single empty line")?;

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut i = 1;
    while i < start_positions.lines().next().unwrap().len() {
        stacks.push(Vec::new());
        i += 4;
    }

    for line in start_positions.lines() {
        for i in 0..stacks.len() {
            // push if not " "
            if line.chars().nth(1+i*4).unwrap() != ' ' {
                stacks[i].push(line.chars().nth(1+i*4).unwrap());
            }
        }
    }

    // revert the stacks
    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    println!("Stacks: {:?}", stacks);

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in instructions.lines().skip(1) {
        let captures = re.captures(line).ok_or("invalid instruction")?;
        let n = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();

        // Step 1
        // let mut i = 0;
        // while i < n {
        //     let to_move = stacks[from-1].pop().unwrap();
        //     stacks[to-1].push(to_move);
        //     i += 1;
        // }
        // Step 2
        let mut i = 0;
        let mut to_move: Vec<char> = Vec::new();
        while i < n {
            to_move.push(stacks[from-1].pop().unwrap());
            i += 1;
        }
        to_move.reverse();
        for c in to_move {
            stacks[to-1].push(c);
        }
    }

    // get the top element of each stack and print it
    let mut result = String::new();
    for stack in stacks {
        result.push(stack[stack.len()-1]);
    }

    println!("{:?}", result);

    
    Ok(())
}