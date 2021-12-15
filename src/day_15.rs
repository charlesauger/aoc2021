use crate::helpers::read_lines;
use std::collections::{HashSet};

pub fn day_15_part_1() {
    if let Ok(file_lines) = read_lines("resources/day_15.txt") {

        // const WIDTH: usize = 10;
        // const HEIGHT: usize = 10;
        const WIDTH: usize = 100;
        const HEIGHT: usize = 100;

        let mut cave = [[0 as usize; WIDTH]; HEIGHT];
        
        let mut x = 0;
        for file_line in file_lines {
            let line = file_line.unwrap();

            let mut y = 0;
            for c in line.chars() {
                cave[x][y] = c.to_digit(10).unwrap() as usize;

                y +=1;
            }

            x += 1;
        }

        // println!("{:?}", cave);

        // Do Djikstra's algorithm
        let mut to_search = HashSet::<(usize, usize)>::new();
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                to_search.insert((i, j));
            }
        }

        // May as well set this to 0 since we should never come back to it
        let mut distance = [[usize::MAX; WIDTH]; HEIGHT];
        distance[0][0] = 0;

        while !to_search.is_empty() {
            let mut curr = (0, 0);
            let mut lowest_coord_value = usize::MAX;
            for i in 0..WIDTH {
                for j in 0..HEIGHT {
                    if to_search.contains(&(i, j)) && distance[i][j] < lowest_coord_value {
                        curr = (i, j);
                        lowest_coord_value = distance[i][j];
                    }
                }
            }

            // Pop the current coordinate
            to_search.remove(&curr);

            // Search through all adjacent positions and update their distance
            let adjacent_positions = find_valid_adjacent_positions(curr.0, curr.1, WIDTH, HEIGHT);

            for pos in adjacent_positions {
                if to_search.contains(&pos) {
                    let new_distance = distance[curr.0][curr.1] + cave[pos.0][pos.1];

                    if new_distance < distance[pos.0][pos.1] {
                        distance[pos.0][pos.1] = new_distance;
                    }
                }
            }
        }

        // println!("{:?}", distance);
        // println!("{:?}", distance[WIDTH-1][HEIGHT-1]);

        println!("Shortest distance to the lower right corner = {}", distance[WIDTH-1][HEIGHT-1]);
    }
}

fn find_valid_adjacent_positions(i: usize, j: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut valid_positions = Vec::<(usize, usize)>::new();
    
    if i > 0 {
        valid_positions.push((i-1, j));
    }

    if j -1 > 0 {
        valid_positions.push((i, j-1));
    }

    if i < (h - 1) {
        valid_positions.push((i+1, j));
    }

    if j < (w - 1) {
        valid_positions.push((i, j+1));
    }

    valid_positions
}

