use crate::helpers::read_lines;
use std::collections::{HashSet};
use priority_queue::DoublePriorityQueue;

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

pub fn day_15_part_2() {
    if let Ok(file_lines) = read_lines("resources/day_15.txt") {

        // const WIDTH: usize = 10;
        // const HEIGHT: usize = 10;
        const WIDTH: usize = 100;
        const HEIGHT: usize = 100;

        let mut cave = vec![[0 as usize; WIDTH * 5]; HEIGHT * 5];
        
        let mut x = 0;
        for file_line in file_lines {
            let line = file_line.unwrap();

            let mut y = 0;
            for c in line.chars() {
                let number = c.to_digit(10).unwrap() as usize;

                for i in 0..5 {
                    for j in 0..5 {
                        let mut new_number = number + i + j;
                        if new_number > 9 {
                            new_number = new_number % 10 + 1;
                        }

                        let x_pos = x + (WIDTH * i);
                        let y_pos = y + (HEIGHT * j);
                        cave[x_pos][y_pos] = new_number;
                    }
                }

                y +=1;
            }

            x += 1;
        }

        for i in 0..HEIGHT*5 {
            for j in 0..WIDTH * 5 {
                print!("{}", cave[i][j]);
            }
            println!();
        }

        // Do Djikstra's algorithm

        let mut to_search = DoublePriorityQueue::<(usize, usize), usize>::new();
        for i in 0..WIDTH*5 {
            for j in 0..HEIGHT*5 {
                to_search.push((i, j), usize::MAX);
            }
        }

        // May as well set this to 0 since we should never come back to it
        let mut distance = vec![[usize::MAX; WIDTH*5]; HEIGHT*5];
        distance[0][0] = 0;

        to_search.change_priority(&(0,0), distance[0][0]);

        while !to_search.is_empty() {
            // Pop the current minimum
            let curr = to_search.pop_min().unwrap();

            // Search through all adjacent positions and update their distance
            let adjacent_positions = find_valid_adjacent_positions(curr.0.0, curr.0.1, WIDTH*5, HEIGHT*5);

            for pos in adjacent_positions {
                let adj_pos_prio = to_search.get(&pos);
                if adj_pos_prio != None {
                    let new_distance: usize = distance[curr.0.0][curr.0.1] + cave[pos.0][pos.1];
    
                    if new_distance < distance[pos.0][pos.1] {
                        distance[pos.0][pos.1] = new_distance;
    
                        to_search.change_priority(&pos, new_distance);
                    }
                }
            }
        }

        println!("Shortest distance to the lower right corner = {}", distance[(WIDTH*5)-1][(HEIGHT*5)-1]);
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

