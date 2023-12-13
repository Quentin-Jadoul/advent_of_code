use crate:: input;
use ndarray::Array2;

pub fn day13() -> input::Result<()> {
    let content = input::load_day_file("day13.txt");

    // Split the content on empty lines
    let groups: Vec<&str> = content.split("\r\n\r\n").collect();

    println!("Part 1: {}", solve(&groups, false));
    println!("Part 2: {}", solve(&groups, true));
    
    Ok(())
}

fn find_symmetry(array: &Array2<u8>, axis: usize, diffs_required: usize) -> isize {
    let array = if axis == 1 {
        array.t().to_owned()
    } else {
        array.clone()
    };

    let len = array.nrows();
    for row_num in 0..(len - 1) {
        let diffs = array.row(row_num).iter().zip(array.row(row_num + 1).iter()).filter(|&(a, b)| a != b).count();
        if diffs <= diffs_required {
            let rows_after = len - (row_num + 2);
            let mut diffs = diffs;
            for i in 0..rows_after {
                if row_num as isize - i as isize == 0 {
                    break;
                }
                diffs += array.row(row_num - 1 - i).iter().zip(array.row(row_num + 2 + i).iter()).filter(|&(a, b)| a != b).count();
                if diffs > diffs_required {
                    break;
                }
            }
            if diffs == diffs_required {
                return row_num as isize;
            }
        }
    }
    -1
}

fn solve(groups: &Vec<&str>, is_part_2: bool) -> isize {
    let diffs_required = if is_part_2 { 1 } else { 0 };

    let mut sum = 0;

    for group in groups {
        // Store content in a 2D array
        let flat_map: Vec<u8> = group
            .lines()
            .flat_map(|line| {
                line.chars()
                    .map(|ch| if ch == '#' { 1 } else { 0 })
            })
            .collect();
    
        let rows = group.lines().count();
        let cols = group.lines().next().unwrap().len();
    
        let map: Array2<u8> = Array2::from_shape_vec((rows, cols), flat_map).unwrap();
    
        sum += 100 * (find_symmetry(&map, 0, diffs_required) + 1);
        sum += find_symmetry(&map, 1, diffs_required) + 1;
    }
    sum
}