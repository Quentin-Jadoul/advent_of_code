use std::collections::HashMap;

use crate:: input;

pub fn day18() -> input::Result<()> {
    println!("Step 1: {}", part_1());
    
    Ok(())
}

pub fn part_1() -> usize {
    // the coordinates are in a 3D space, they repreesent the position of cubes of 1x1x1
    // the cubes are in a grid, the grid is infinite
    // we want to know the final surface of the 3d object
    let mut coords = parse();
    let mut explored_coords: HashMap<Coordinates, u8> = HashMap::new();

    let mut sum: u32 = 0;
    // we check each coordinate and the hashmap
    for mut coord in coords.keys() {
        // update the value of the coordinate
        let val = check_around(&coord, &coords, explored_coords);
        // update the hashmap
        sum += val as u32;
    }
    sum as usize
}

pub fn part_2() -> usize {
    0
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct  Coordinates {
    x:  u32 ,
    y:  u32 ,
    z:  u32 ,
}

fn parse() -> HashMap<Coordinates, u8> {
    let contents = input::load_day_file("day18.txt");
    //let mut coords: Vec<Coordinates> = Vec::new();
    // create a hashmap containing the coordinates
    let mut coords: HashMap<Coordinates, u8> = HashMap::new();

    for line in contents.lines() {
        let mut parts = line.split(",");
        let x = parts.next().unwrap().parse::<u32>().unwrap();
        let y = parts.next().unwrap().parse::<u32>().unwrap();
        let z = parts.next().unwrap().parse::<u32>().unwrap();
        coords.insert(Coordinates { x, y, z }, 6);
    }
    coords
}

fn check_around(coords: &Coordinates, map: &HashMap<Coordinates, u8>, explored_coords: HashMap<Coordinates, u8>) -> u8 {
    // check the 6 directions around the coordinates
    // if the coordinates are in the hashmap, remove 1 from the value
    // if the coordinates are not in the hashmap, do nothing
    let mut count = 6;

    // up
    let up = Coordinates { x: coords.x, y: coords.y + 1, z: coords.z };
    if map.contains_key(&up) {
        count -= 1;
    }
    explored_coords.insert(*coords, count);
    // down
    if coords.y > 0 {
        let down = Coordinates { x: coords.x, y: coords.y - 1, z: coords.z };
        if map.contains_key(&down) {
            count -= 1;
        }
    }
    // left
    if coords.x > 0 {
        let left = Coordinates { x: coords.x - 1, y: coords.y, z: coords.z };
        if map.contains_key(&left) {
            count -= 1;
        }
    }
    // right
    let right = Coordinates { x: coords.x + 1, y: coords.y, z: coords.z };
    if map.contains_key(&right) {
        count -= 1;
    }
    // front
    let front = Coordinates { x: coords.x, y: coords.y, z: coords.z + 1 };
    if map.contains_key(&front) {
        count -= 1;
    }
    // back
    if coords.z > 0 {
        let back = Coordinates { x: coords.x, y: coords.y, z: coords.z - 1 };
        if map.contains_key(&back) {
            count -= 1;
        }
    }
    count

}