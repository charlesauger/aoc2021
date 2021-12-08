use crate::helpers::read_lines;

// 1000 elements in the input
// Lowest element: 0
// Highest element: 1898

pub fn day_7_part_1() {
    if let Ok(mut file_lines) = read_lines("resources/day_7.txt") {
        let file_line = file_lines.next().unwrap().unwrap();

        let split = file_line.split(",").collect::<Vec<&str>>();
        println!("{}", split.len());

        let mut distances: [i32; 1000] = [0; 1000];
        let mut i = 0;
        for s in split {
            distances[i] = s.parse::<i32>().unwrap();
            i += 1;
        }

        // let mut lowest = std::i32::MAX;
        // let mut highest = std::i32::MIN;

        // for d in distances {
        //     if d < lowest {
        //         lowest = d;
        //     }

        //     if d > highest {
        //         highest = d;
        //     }
        // }

        // println!("Lowest number: {}", lowest);
        // println!("Highest number: {}", highest);

        let lowest = 0;
        let highest = 1898;

        // let positions_to_test = [0; 1898];
        let mut best_position = 0;
        let mut best_position_difference = std::i32::MAX;
        
        for position_to_test in 0..=1898 {
            let mut current_position_difference = 0;

            for d in distances {
                current_position_difference += (d - position_to_test as i32).abs();
            }

            if current_position_difference < best_position_difference {
                best_position = position_to_test;
                best_position_difference = current_position_difference;
            }
        }

        println!("Best position to move to = {}", best_position);
        println!("Best position difference = {}", best_position_difference);
    }
}

pub fn day_7_part_2() {
    if let Ok(mut file_lines) = read_lines("resources/day_7.txt") {
        let file_line = file_lines.next().unwrap().unwrap();

        let split = file_line.split(",").collect::<Vec<&str>>();
        println!("{}", split.len());

        let mut distances: [i64; 1000] = [0; 1000];
        let mut i = 0;
        for s in split {
            distances[i] = s.parse::<i64>().unwrap();
            i += 1;
        }

        let lowest = 0;
        let highest = 1898;

        let mut best_position = 0;
        let mut best_position_difference = std::i64::MAX;
        
        for position_to_test in 0..=1898 {
            let mut current_position_difference = 0;

            for d in distances {
                if d == position_to_test {
                    current_position_difference += 0;
                } else {
                    let n = (d - position_to_test).abs();
                    let low: i64 = 1;
                    let high = n;
                    let consecutive_sum = (n as f64 / 2.0) * (low + high) as f64;
                    let consecutive_sum_int = consecutive_sum as i64;

                    current_position_difference += consecutive_sum_int;
                }
            }

            if current_position_difference < best_position_difference {
                best_position = position_to_test;
                best_position_difference = current_position_difference;
            }
        }

        println!("Best position to move to = {}", best_position);
        println!("Best position difference = {}", best_position_difference);
    }
}
