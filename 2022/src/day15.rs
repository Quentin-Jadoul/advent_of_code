use std::collections::HashMap;

use regex::Regex;

use crate:: input;

struct Sensor {
    x: i32,
    y: i32,
    closest_beacon: Beacon,
}

struct Beacon {
    x: i32,
    y: i32
}

pub fn day15() -> input::Result<()> {
    println!("Part 1: {:?}",part_1());
    println!("Part 2: {:?}",part_2());
    Ok(())
}

pub fn part_1() -> usize {
    let sensors = parse();
    
    let mut cave_line = HashMap::new();
    for sensor in &sensors {
        // if a beacon or sensor is on line 10, add it to the hashset
        if sensor.y == 2000000 {
            cave_line.insert(sensor.x, "S");
        }
        if sensor.closest_beacon.y == 2000000 {
            cave_line.insert(sensor.closest_beacon.x, "B");
        }
        // calculate the distance between the sensor and the closest beacon
        let closest_beacon_distance = (sensor.x - sensor.closest_beacon.x).abs() + (sensor.y - sensor.closest_beacon.y).abs();
        let d = closest_beacon_distance - (sensor.y - 2000000).abs();
        if d > 0 {
            cave_line.insert(sensor.x, "#");
            for x in 0..d + 1 {
                // insert if the key is not already present
                cave_line.entry(sensor.x + x).or_insert("#");
                cave_line.entry(sensor.x - x).or_insert("#");
            }
        }
    }
    // count the number of # inside the hashset
    let mut count = 0;
    for (_, value) in cave_line {
        if value == "#" {
            count += 1;
        }
    }
    count
}

pub fn part_2() -> usize {
    let sensors = parse();
    
    for row in 0..4_000_001 {
        //let mut intervals: HashSet<[u64; 2]> = HashSet::new();
        let mut intervals: Vec<[u64; 2]> = Vec::new();
        for sensor in &sensors {
            // calculate the distance between the sensor and the closest beacon
            let closest_beacon_distance = (sensor.x - sensor.closest_beacon.x).abs() + (sensor.y - sensor.closest_beacon.y).abs();
            let d = closest_beacon_distance - (sensor.y - row).abs();
            if d > 0 {
                let left = if sensor.x - d > 0 {
                    sensor.x - d
                } else {
                    0
                };
                let right = if sensor.x + d < 4_000_000 {
                    sensor.x + d
                } else {
                    4_000_000
                };
                intervals.push([left as u64, right as u64]);
            }
        }
        intervals = merge_intervals(intervals);

        if intervals.len() == 2 {
            let x = intervals[0][1].min(intervals[1][1]) + 1;
            let result = x * 4_000_000 + row as u64;
            return result as usize
        }
        if row % 40_000 == 0 {
            println!("row: {}", row/40_000);
        }
    }
    unreachable!("No solution found")
}

fn merge_intervals(mut intervals: Vec<[u64;2]>) -> Vec<[u64;2]> {
    while intervals.len() > 2 {
        let mut to_remove: Vec<usize> = Vec::new();
        let mut to_add: Vec<[u64;2]> = Vec::new();
        let length = intervals.len();
        for i in 0..length {
            for j in 0..length {
                if i != j {
                    let interval1 = intervals[i];
                    let interval2 = intervals[j];
                    if interval1[1].min(interval2[1]) + 1 > interval1[0].max(interval2[0]) {
                        // combine the intervals
                        let new_interval = [interval1[0].min(interval2[0]), interval1[1].max(interval2[1])];
                        // add the new interval and remove the old one
                        // remove two element at the same time
                        to_remove.push(i);
                        to_remove.push(j);
                        to_add.push(new_interval);
                        break;
                    } 
                }
            }
        }
        // remove the old intervals
        to_remove.sort();
        to_remove.dedup();
        to_remove.reverse();
        for i in to_remove {
            intervals.remove(i);
        }
        to_add.sort();
        to_add.dedup();
        for interval in to_add {
            intervals.push(interval);
        }
        intervals.dedup();
    }
    if intervals.len() == 2 {
        // check if the intervals overlap
        if intervals[0][1].min(intervals[1][1]) + 1 > intervals[0][0].max(intervals[1][0]) {
            // combine the intervals
            let new_interval = [intervals[0][0].min(intervals[1][0]), intervals[0][1].max(intervals[1][1])];
            // add the new interval and remove the old one
            intervals = Vec::new();
            intervals.push(new_interval);
            intervals.dedup();
        }
    }
    intervals
}

fn parse() -> Vec<Sensor> {
    let contents = input::load_day_file("day15.txt");

    // create a regex
    let re = Regex::new(r"(?m)Sensor at x=(-\d+|\d+), y=(-\d+|\d+): closest beacon is at x=(-\d+|\d+), y=(-\d+|\d+)").unwrap();
    
    contents.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        let sensor_x = caps[1].parse::<i32>().unwrap();
        let sensor_y = caps[2].parse::<i32>().unwrap();
        let beacon_x = caps[3].parse::<i32>().unwrap();
        let beacon_y = caps[4].parse::<i32>().unwrap();

        Sensor {
            x : sensor_x,
            y : sensor_y,
            closest_beacon: Beacon {
                x: beacon_x,
                y: beacon_y
            }
        }
    }).collect()
}