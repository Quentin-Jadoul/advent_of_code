use crate::input;

pub fn day10() -> input::Result<()> {
    let contents = input::load_day_file("day10.txt");

    let mut number_of_cycle: usize = 0;
    let mut x_value: i32 = 1; // position of the center of the sprite
    let mut signal_strength = 0;
    let mut screen: Vec<Vec<&str>> = vec![vec!["."; 40]; 6];

    for line in contents.lines() {
        let mut instruction = line.split_whitespace();

        // match on the first word of the instruction
        match instruction.next() {
            Some("noop") => {
                number_of_cycle += 1;
                step1(number_of_cycle.clone(), x_value.clone(), &mut signal_strength);
                step2(x_value.clone(), number_of_cycle.clone(), &mut screen);
            }
            Some("addx") => {
                number_of_cycle += 1;
                step1(number_of_cycle.clone(), x_value.clone(), &mut signal_strength);
                step2(x_value.clone(), number_of_cycle.clone(), &mut screen);
                number_of_cycle += 1;
                step1(number_of_cycle.clone(), x_value.clone(), &mut signal_strength);
                x_value += instruction.next().unwrap().parse::<i32>().unwrap();
                step2(x_value.clone(), number_of_cycle.clone(), &mut screen);
            }
            _ => {
                println!("Instruction not found");
            }
        }
    }
    println!("Step 1: {}", signal_strength);
    println!("Step 2:");
    for row in screen {
        for column in row {
            print!("{}", column);
        }
        println!("");
    }

    Ok(())
}

pub fn step2(x_value: i32, number_of_cycle: usize, screen: &mut Vec<Vec<&str>>) {
    let row = number_of_cycle / 40;
    let column = number_of_cycle % 40;
    if row < 6 {
        if column as i32 == x_value || column as i32 == x_value + 1 || column as i32 == x_value - 1 {
            screen[row][column] = "#";
        } else {
            screen[row][column] = ".";
        }
    }
}

pub fn step1(number_of_cycle: usize, x_value: i32, signal_strength:&mut i32) {
    if number_of_cycle >= 20 {
        if (number_of_cycle - 20) % 40 == 0 {
            *signal_strength += number_of_cycle as i32 * x_value;
        }
    }
}