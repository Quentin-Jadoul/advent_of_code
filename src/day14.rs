use std::collections::HashSet;
use itertools::Itertools;

use crate:: input;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn neighbours(&self) -> [Coord; 3] {
        let down = Coord {
            x: self.x,
            y: self.y + 1,
        };
        let down_left = Coord {
            x: self.x - 1,
            y: self.y + 1,
        };
        let down_right = Coord {
            x: self.x + 1,
            y: self.y + 1,
        };
        [down, down_left, down_right]
    }

    fn next(&self, cave: &[Vec<Tile>]) -> Option<Coord> {
        self.neighbours()
            .into_iter()
            .find(|coord| cave[coord.y as usize][coord.x as usize] == Tile::Empty)
    }

    fn next_2(&self, cave: &[Vec<Tile>], floor_y: i32) -> Option<Coord> {
        if self.y + 1 == floor_y {
            return None;
        }

        self.neighbours()
            .into_iter()
            .find(|coord| cave[coord.y as usize][coord.x as usize] == Tile::Empty)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Rock,
    Sand,
}

pub fn day14() -> input::Result<()> {
    println!("Part 1: {}",part_1());
    println!("Part 2: {}",part_2());

    Ok(())
}

pub fn part_1() -> usize {
    let rock_lines = parse();
    let rocks = rocks_in_cave(rock_lines);

    let start = Coord { x: 500, y: 0 };
    let max_y = rocks.iter().map(|c| c.y).max().unwrap();

    let width = 500 + max_y + 2;

    let mut cave = vec![vec![Tile::Empty; width as usize]; (max_y + 2) as usize];

    for pos in rocks {
        cave[pos.y as usize][pos.x as usize] = Tile::Rock;
    }

    for i in 0.. {
        let mut sand = start;

        while let Some(next_empty_coord) = sand.next(&cave) {
            sand = next_empty_coord;
            if sand.y > max_y {
                return i;
            }
        }
        cave[sand.y as usize][sand.x as usize] = Tile::Sand;
    }
    unreachable!()
}

pub fn part_2() -> usize {
    let rock_lines = parse();
    let rocks = rocks_in_cave(rock_lines);

    let start = Coord { x: 500, y: 0 };
    let max_y = rocks.iter().map(|c| c.y).max().unwrap();

    let width = 500 + max_y + 2;
    let floor_y = max_y + 2;

    let mut cave = vec![vec![Tile::Empty; width as usize]; (max_y + 2) as usize];

    for pos in rocks {
        cave[pos.y as usize][pos.x as usize] = Tile::Rock;
    }

    for i in 0.. {
        let mut sand = start;

        while let Some(next_empty_coord) = sand.next_2(&cave, floor_y) {
            sand = next_empty_coord;
        }
        cave[sand.y as usize][sand.x as usize] = Tile::Sand;

        if sand == start {
            return i + 1;
        }
    }
    unreachable!()
}

fn parse() -> Vec<Vec<Coord>> {
    let contents = input::load_day_file("day14.txt");

    contents.lines().map(|line| {
        line.split(" -> ").map(|coords| {
            let (x,y) = coords.split_once(",").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            Coord {x, y}
        })
        .collect()
    })
    .collect()
}

fn rocks_in_cave(rock_lines: Vec<Vec<Coord>>) -> HashSet<Coord> {
    rock_lines.iter().flat_map(|path| {
        path.iter().tuple_windows().flat_map(|(start, end)| {
            let diff_x = end.x - start.x;
            let diff_y = end.y - start.y;
            let direction = Coord {
                x: diff_x.signum(),
                y: diff_y.signum(),
            };
            let amount = diff_x.abs().max(diff_y.abs()) as usize;

            (0..=amount).map(move |amount| {
                let diff_x = amount as i32 * direction.x;
                let diff_y = amount as i32 * direction.y;

                Coord {
                    x: start.x + diff_x,
                    y: start.y + diff_y,
                }
            })
        })
    })
    .collect()
} 


