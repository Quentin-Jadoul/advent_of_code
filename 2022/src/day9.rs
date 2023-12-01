use std::collections::HashSet;

use crate::input;

pub fn day9() -> input::Result<()> {
    let contents = input::load_day_file("day9.txt");

    let mut explored_coordinates: HashSet<(i32,i32)> = HashSet::new();

    // old way for step 1
    // // create a vector of coordinates, it start at 0,0
    // let mut head_position: (i32,i32) = (0,0);
    // let mut tail_position: (i32,i32) = (0,0);

    // Step 1
    // let mut knots_positions: [(i32,i32);2] = [(0,0);2];
    // Step 2
    let mut knots_positions: [(i32,i32);10] = [(0,0);10];

    for line in contents.lines() {
        let mut line = line.split_whitespace();
        // get the direction
        let direction = line.next().ok_or("no direction found")?;
        // get the distance
        let distance = line.next().ok_or("no distance found")?.parse::<i32>()?;

        for _i in 0..distance {
            match direction {
                "U" => knots_positions[0].1 += 1,
                "D" => knots_positions[0].1 -= 1,
                "R" => knots_positions[0].0 += 1,
                "L" => knots_positions[0].0 -= 1,
                _ => return Err("invalid direction".into()),
            }

            for i in 1..knots_positions.len() {
                knots_positions[i] = move_tail(&knots_positions[i -1], &knots_positions[i]);
            }
            explored_coordinates.insert(knots_positions.last().unwrap().clone());
            
            // old way for step 1
            // // if the tail is further than 1 step (horizontal ,vertical or diagonal) from the head, move it
            // if (head_position.0 - tail_position.0).abs() > 1 || (head_position.1 - tail_position.1).abs() > 1 {
            //     match direction {
            //         "U" => tail_position = (head_position.0, head_position.1 - 1),
            //         "D" => tail_position = (head_position.0, head_position.1 + 1),
            //         "R" => tail_position = (head_position.0 - 1, head_position.1),
            //         "L" => tail_position = (head_position.0 + 1, head_position.1),
            //         _ => return Err("invalid direction".into()),
            //     }
            // }
            // explored_coordinates.insert(tail_position.clone());

        }    
    }
    let result = explored_coordinates.len();
    println!("{}",result);
    
    Ok(())
}

// Create a function that take the positions of the head and tail and return where to move the tail
fn move_tail(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    if (dx.abs() > 1 && dy.abs() == 0 ) || (dy.abs() > 1 && dx.abs() == 0 ) {
        (tail.0 + dx/2 , tail.1 + dy/2)
    } else if dx.abs() > 1 || dy.abs() > 1 {
        let mut around_head: HashSet<(i32,i32)> = HashSet::new();
        let mut around_tail: HashSet<(i32,i32)> = HashSet::new();
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                around_head.insert((head.0 + i, head.1 + j));
                if i != 0 && j != 0 {
                    around_tail.insert((tail.0 + i, tail.1 + j));
                }
            }
        }
        *around_head.intersection(&around_tail).next().unwrap()
    } else {
        tail.clone()
    }
}