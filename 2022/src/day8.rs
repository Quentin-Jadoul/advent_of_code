use crate::input;

pub fn day8() -> input::Result<()> {
    let contents = input::load_day_file("exemple.txt");

    let width = contents.lines().next().ok_or("no line found")?.len();
    let height = contents.len() / width - 1;

    let mut visible_trees = 2 * width + 2 * height - 4;

    let mut scenic_score: Vec<u32> = Vec::new();

    let mut matrix: Vec<Vec<u8>> = Vec::new();

    for line in contents.lines() {
        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).ok_or("not a digit")? as u8);
        }
        matrix.push(row);
    }

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let left = matrix[i][0..j].to_vec();
            let right = matrix[i][j+1..].to_vec();
            let up: Vec<u8> = matrix.iter().map(|x| x[j]).take(i).collect();
            let down: Vec<u8> = matrix.iter().map(|x| x[j]).rev().take(height - i - 1).collect();
            // check if the current digit is bigger than all the digits in a given direction
            if matrix[i][j] > *left.iter().max().unwrap() || matrix[i][j] > *right.iter().max().unwrap() || matrix[i][j] > *up.iter().max().unwrap() || matrix[i][j] > *down.iter().max().unwrap() {
                visible_trees += 1;
            }
            //step 2
            let mut up_visible_trees = 0;
            let mut down_visible_trees = 0;
            let mut left_visible_trees = 0;
            let mut right_visible_trees = 0;
            for k in left.iter().rev() {
                if k < &matrix[i][j] {
                    left_visible_trees += 1;
                } else if k == &matrix[i][j] {
                    left_visible_trees += 1;
                    break;
                } else {
                    break;
                }
            }
            for k in right.iter() {
                if k < &matrix[i][j] {
                    right_visible_trees += 1;
                } else if k == &matrix[i][j] {
                    right_visible_trees += 1;
                    break;
                } else {
                    break;
                }
            }
            for k in up.iter().rev() {
                if k < &matrix[i][j] {
                    up_visible_trees += 1;
                } else if k == &matrix[i][j] {
                    up_visible_trees += 1;
                    break;
                } else {
                    break;
                }
            }
            for k in down.iter().rev() {
                if k < &matrix[i][j] {
                    down_visible_trees += 1;
                } else if k == &matrix[i][j] {
                    down_visible_trees += 1;
                    break;
                } else {
                    break;
                }
            }
            scenic_score.push(left_visible_trees * right_visible_trees * up_visible_trees * down_visible_trees);
        }
    }

    println!("Result: {}", visible_trees);
    println!("Scenic_score: {}", scenic_score.iter().max().unwrap());

    Ok(())
}