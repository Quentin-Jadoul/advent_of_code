use crate:: input;

pub fn day2() -> input::Result<()> {
    

    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
    
    Ok(())
}

pub fn part_1() -> usize {
    let contents = input::load_day_file("day2.txt");

    let mut sum = 0;

    let red = 12;
    let green = 13;
    let blue = 14;
    
    'outer: for line in contents.lines() {
        let mut possible = true;
        // Break on the : character
        let (id, game) = line.split_once(":").unwrap();
        let id = id.split_whitespace().nth(1).unwrap();

        let rounds = game.split(";").collect::<Vec<&str>>();
        for round in rounds {
            for part in round.split(",") {
                let (number, color) = part.trim().split_once(" ").unwrap();
                let number = number.parse::<usize>().unwrap();
                match color {
                    "red" => if number > red { possible = false; },
                    "green" => if number > green { possible = false; },
                    "blue" => if number > blue { possible = false; },
                    _ => (),
                }
                if !possible {
                    continue 'outer;
                }
            }
        }
        if possible {
            sum += id.parse::<usize>().unwrap();
        }
    }
    sum
}
pub fn part_2() -> usize {
    let contents = input::load_day_file("day2.txt");

    let mut sum = 0;
    
    for line in contents.lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        // Break on the : character
        let (_id, game) = line.split_once(":").unwrap();

        let rounds = game.split(";").collect::<Vec<&str>>();
        for round in rounds {
            let parts = round.split(",").collect::<Vec<&str>>();
            for part in parts {
                let (number, color) = part.trim().split_once(" ").unwrap();
                let number = number.parse::<usize>().unwrap();
                match color {
                    "red" => if number > red { red = number },
                    "green" => if number > green { green = number },
                    "blue" => if number > blue { blue = number },
                    _ => (),
                }
            }
        }
        sum += red * green * blue;
    }
    sum
}