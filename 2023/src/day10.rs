use std::{thread, time};

use crate:: input;

pub fn day10() -> input::Result<()> {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());

    let content = input::load_day_file("day10.txt");

    let mut start_pos = (0, 0);

    // Store it inside a two dimensional grid. grid[row][column]
    let mut map = Vec::new();
    for line in content.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }

    // Create a map to keep track of explored cells
    let mut explored = vec![vec![false; map[0].len()]; map.len()];

    // Find the starting position (column, row)
    let mut start_pos = (0, 0);
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start_pos = (j, i);
                break;
            }
        }
    }

    let distance = measure_path(&map, start_pos, &mut explored);

    display_map(&map, &explored);

    println!("distance {:?}", distance);

    // find the middle of the path
    let middle = distance / 2;
    println!("Part 1: {}", middle);

    
    Ok(())
}

pub fn part_1() -> usize {
    0
}
pub fn part_2() -> usize {
    0
}

pub fn measure_path(map: &Vec<Vec<char>>, start_pos: (usize, usize), explored: &mut Vec<Vec<bool>>) -> usize {
    let initial_position = start_pos;
    let mut current_position = get_start_next_pos(&map, start_pos);
    let mut previous_position = start_pos;
    let mut nb_steps = 1;

    while current_position != initial_position {
        let next_pos = next_position(current_position, previous_position, &map);
        nb_steps += 1;

        // Mark the current cell as explored
        let (x, y) = current_position;
        explored[y][x] = true;

        previous_position = current_position;
        current_position = next_pos;
    }
    nb_steps
}

pub fn next_position(current_position: (usize, usize), previous_position: (usize, usize), map: &Vec<Vec<char>>) -> (usize, usize) {
    // We match the current position to find the next one
    let (x, y) = current_position;
    let pipe = map[y][x];

    let direction = (
        x as i32 - previous_position.0 as i32,
        y as i32 - previous_position.1 as i32,
    );

    let up = (x, y.checked_sub(1).unwrap_or(y));
    let down = (x, y.checked_add(1).unwrap_or(y));
    let left = (x.checked_sub(1).unwrap_or(x), y);
    let right = (x.checked_add(1).unwrap_or(x), y);

    match pipe {
        '-' => if direction.0 > 0 { right } else { left },
        'L' => if direction.0 != 0 { up } else { right },
        'F' => if direction.0 != 0 { down } else { right },
        '|' => if direction.1 > 0 { down } else { up },
        'J' => if direction.0 != 0 { up } else { left },
        '7' => if direction.0 != 0 { down } else { left },
        _ => {
            println!("Error: {:?}", pipe);
            current_position
        }
    }
}

pub fn get_start_next_pos(map: &Vec<Vec<char>>, start_pos: (usize, usize)) -> (usize, usize) {
    let (x, y) = start_pos;
    match (x.checked_sub(1), x.checked_add(1), y.checked_sub(1), y.checked_add(1)) {
        (Some(x_left), _, _, _) if ['-', 'L', 'F'].contains(&map[y][x_left]) => (x_left, y),
        (_, Some(x_right), _, _) if ['-', 'J', '7'].contains(&map[y][x_right]) => (x_right, y),
        (_, _, Some(y_up), _) if ['|', '7', 'F'].contains(&map[y_up][x]) => (x, y_up),
        (_, _, _, Some(y_down)) if ['|', 'J', 'L'].contains(&map[y_down][x]) => (x, y_down),
        _ => start_pos,
    }
}

use crossterm::{cursor, execute, style, terminal, Result};
use std::io::{stdout, Write};

fn display_map(map: &Vec<Vec<char>>, explored: &Vec<Vec<bool>>) -> Result<()> {
    let mut stdout = stdout();

    // Clear the screen
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    // Move the cursor to the top left
    execute!(stdout, cursor::MoveTo(0, 0))?;

    // Find the maximum row length
    let max_width = map.iter().map(|row| row.len()).max().unwrap_or(0);

    for (row, explored_row) in map.iter().zip(explored) {
        let row_string: String = row.iter().collect();
        let padded_row_string = format!("{:<width$}", row_string, width = max_width);

        for (cell, &explored) in padded_row_string.chars().zip(explored_row) {
            // Use green for explored cells and white for unexplored cells
            let color = if explored { style::Color::Green } else { style::Color::White };

            // Print the cell with the chosen color
            execute!(stdout, style::SetForegroundColor(color))?;
            write!(stdout, "{}", cell)?;
            execute!(stdout, style::ResetColor)?;
        }
        writeln!(stdout)?;
    }

    Ok(())
}