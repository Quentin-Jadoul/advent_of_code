use crate:: input;

pub fn day10() -> input::Result<()> {
    let content = input::load_day_file("day10.txt");

    // Store it inside a two dimensional grid. grid[row][column]
    let map: Vec<Vec<_>> = content.lines().map(|line| line.chars().collect()).collect();

    // Find the starting position (column, row)
    let start_pos = map.iter().enumerate()
        .find_map(|(i, row)| 
            row.iter().enumerate()
                .find(|&(_, &cell)| cell == 'S')
                .map(|(j, _)| (j, i))
        ).unwrap_or((0, 0));

    let (distance, vertices) = measure_path(&map, start_pos);

    println!("Part 1: {}", distance / 2);

    // convert the vertices to f64
    let vertices: Vec<(f64, f64)> = vertices.iter().map(|&(x, y)| (x as f64, y as f64)).collect();

    // calculate the inverse pick formula
    let i = shoelace(&vertices) as usize - (vertices.len()/2) + 1;
    println!("Part 2: {}", i);

    Ok(())
}

pub fn measure_path(map: &Vec<Vec<char>>, start_pos: (usize, usize)) -> (usize, Vec<(usize, usize)>) {
    let initial_position = start_pos;
    let mut current_position = get_start_next_pos(&map, start_pos);
    let mut previous_position = start_pos;
    let mut nb_steps = 1;
    let mut vertices = vec![start_pos];

    while current_position != initial_position {
        let next_pos = next_position(current_position, previous_position, &map);
        nb_steps += 1;
        vertices.push(current_position);
        previous_position = current_position;
        current_position = next_pos;
    }
    (nb_steps, vertices)
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

fn shoelace(vertices: &[(f64, f64)]) -> f64 {
    let mut sum = 0.0;
    for i in 0..vertices.len() {
        let (x1, y1) = (vertices[i].0 - 0.5, vertices[i].1 - 0.5);
        let (x2, y2) = (vertices[(i + 1) % vertices.len()].0 - 0.5, vertices[(i + 1) % vertices.len()].1 - 0.5);
        sum += (y1 + y2) * (x2 - x1);
    }
    0.5 * sum.abs()
}