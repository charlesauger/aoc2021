use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_1_part_1() {
    let mut previous = 0;
    let mut increased_count = 0;

    if let Ok(lines) = read_lines("resources/day_1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(num_line) = line {
                let num: i32 = num_line.parse().unwrap();
                if previous > 0 && previous < num {
                    increased_count += 1;
                }

                previous = num;
            }
        }
    }

    println!("{} measurements larger than previous measurement", increased_count);
}

pub fn day_1_part_2() {
    let window_size = 3;
    let mut window = VecDeque::new();
    let mut increased_count = 0;

    if let Ok(lines) = read_lines("resources/day_1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(num_line) = line {

                let num: i32 = num_line.parse().unwrap();
                
                if window.len() == window_size {
                    // Once we have processed 3 elements we can start comparing
                    // the previous sliding window with the current one.

                    if window.pop_front().unwrap() < num {
                        increased_count += 1;
                    }
                }
                window.push_back(num);
            }
        }
    }

    println!("{} measurements larger than previous measurement", increased_count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


