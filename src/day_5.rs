use std::collections::HashMap;
use std::mem;

use crate::helpers::read_lines;

#[derive(Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    pos1: Vec2,
    pos2: Vec2,
}

impl Line {
    fn horizontal(&self) -> bool {
        self.pos1.x == self.pos2.x
    }

    fn vertical(&self) -> bool {
        self.pos1.y == self.pos2.y
    }
}

#[derive(Debug)]
struct Board {
    board: HashMap<(i32, i32), i32>,
}

impl Board {
    fn add_line(&mut self, line: Line) {
        if line.horizontal() {
            let x = line.pos1.x;
            let mut range_start = line.pos1.y;
            let mut range_end = line.pos2.y;
            if range_start > range_end {
                mem::swap(&mut range_start, &mut range_end);
            }
            for y in range_start..=range_end {
                match self.board.get(&(x,y)) {
                    Some(&value) => {
                        self.board.insert((x,y), value + 1)
                    },
                    _ => {
                        self.board.insert((x,y), 1)
                    },
                };
            }
        } else if line.vertical() {
            let y = line.pos1.y;
            let mut range_start = line.pos1.x;
            let mut range_end = line.pos2.x;
            if range_start > range_end {
                mem::swap(&mut range_start, &mut range_end);
            }
            for x in range_start..=range_end {
                match self.board.get(&(x,y)) {
                    Some(&value) => {
                        self.board.insert((x,y), value + 1)
                    },
                    _ => {
                        self.board.insert((x,y), 1)
                    },
                };
            }
        }
    }

    fn draw(&self) {
        for x in 0..10 {
            for y in 0..10 {
                match self.board.get(&(x,y)) {
                    Some(&value) => print!("{} ", value),
                    _ => print!("{} ", 0),
                }
            }
            println!("");
        }
    }
}

pub fn day_5_part_1() {
    let mut lines = Vec::<Line>::new();

    if let Ok(mut file_lines) = read_lines("resources/day_5.txt") {
        for file_line_result in file_lines {
            let file_line = file_line_result.unwrap();

            let mut split = file_line.split("->");
            
            let first = split.next().unwrap();
            let mut first_split = first.trim().split(",");
            let first_vec = Vec2 {
                x: first_split.next().unwrap().parse::<i32>().unwrap(),
                y: first_split.next().unwrap().parse::<i32>().unwrap(),
            };

            let second = split.next().unwrap();
            let mut second_split = second.trim().split(",");
            let second_vec = Vec2 {
                x: second_split.next().unwrap().parse::<i32>().unwrap(),
                y: second_split.next().unwrap().parse::<i32>().unwrap(),
            };

            let line = Line {
                pos1: first_vec,
                pos2: second_vec,
            };

            // For part 1 only use horizontal or vertical lines
            if line.horizontal() || line.vertical() {
                // println!("Line is being added to lines: {:?}", line);
                lines.push(line);
            }
        }
    }

    let mut board = Board {
        board: HashMap::new(),
    };

    println!("----------------------");
    for line in lines {
        // println!("Line is being added to board: {:?}", line);
        board.add_line(line);
    }

    // println!("{:?}", board);
    board.draw();

    let mut at_least_two_lines = 0;

    for (k, v) in board.board {
        if v >= 2 {
            at_least_two_lines += 1;
        }
    }

    println!("The number of positions with at least two intersections is {}", at_least_two_lines);
}
