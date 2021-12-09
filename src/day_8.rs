use crate::helpers::read_lines;

pub fn day_8_part_1() {
    if let Ok(file_lines) = read_lines("resources/day_8.txt") {
        
        let mut numbers = [0; 10];

        for file_line in file_lines {
            let line = file_line.unwrap();

            // println!("{}", line);
            let initial_split = line.split("|");

            let mut inputs = Vec::<&str>::new();
            let mut outputs = Vec::<&str>::new();
            let mut input = true;
            for s in initial_split {
                let trimmed = s.trim();
                for trimmed_s in trimmed.split(" ") {
                    if input {
                        inputs.push(trimmed_s);
                    } else {
                        outputs.push(trimmed_s);
                    }
                }
                input = false;
            }

            for s in &outputs {
                match s.len() {
                    2 => numbers[1] += 1,
                    3 => numbers[7] += 1,
                    4 => numbers[4] += 1,
                    7 => numbers[8] += 1,
                    _ => ()
                }
            }

            // println!("{} {}", inputs.len(), outputs.len());
        }

        println!("{:?}", numbers);

        let total: i32 = numbers.iter().sum();
        println!("Number of 1s, 4s, 7s, and 8s = {}", total);
    }
}

pub fn day_8_part_2() {
    if let Ok(file_lines) = read_lines("resources/day_8.txt") {
        
        let mut final_sum = 0;

        for file_line in file_lines {
            let line = file_line.unwrap();

            // println!("{}", line);
            let initial_split = line.split("|");

            let mut inputs = Vec::<&str>::new();
            let mut outputs = Vec::<&str>::new();
            let mut input = true;
            for s in initial_split {
                let trimmed = s.trim();
                for trimmed_s in trimmed.split(" ") {
                    if input {
                        inputs.push(trimmed_s);
                    } else {
                        outputs.push(trimmed_s);
                    }
                }
                input = false;
            }

            final_sum += solve(inputs, outputs);
        }

        println!("Final sum = {}", final_sum);
    }
}

fn solve(mut inputs: Vec<&str>, mut outputs: Vec<&str>) -> i32 {
    inputs.sort_by(|a, b| a.len().cmp(&b.len()));
    // println!("{:?}", inputs);

    let mut solution: [char; 7] = [' '; 7];

    let all_chars = ['a','b','c','d','e','f','g'];

    let one: Vec<char> = inputs[0].chars().collect();
    let four: Vec<char> = inputs[2].chars().collect();
    let seven: Vec<char> = inputs[1].chars().collect();
    let eight: Vec<char> = inputs[9].chars().collect();

    let five_length_1: Vec<char> = inputs[3].chars().collect();
    let five_length_2: Vec<char> = inputs[4].chars().collect();
    let five_length_3: Vec<char> = inputs[5].chars().collect();

    let six_length_1: Vec<char> = inputs[6].chars().collect();
    let six_length_2: Vec<char> = inputs[7].chars().collect();
    let six_length_3: Vec<char> = inputs[8].chars().collect();

    // Letter in 7 but not in 1 is in the top position (index 0)
    for c in &seven {
        if !one.contains(&c) {
            solution[0] = c.clone();
        }
    }

    // Letter in all five character strings (2, 3, and 5) which is not the top position letter
    // and is not in 4 is the bottom position (index 6)
    for c in &all_chars {
        if c != &solution[0] {
            if five_length_1.contains(&c) && five_length_2.contains(&c) && five_length_3.contains(&c) {
                if !four.contains(&c) {
                    solution[6] = c.clone();
                }
            }
        }
    }

    // Letter in all five character strings (2, 3, and 5) which is not the top position letter
    // and is not the bottom position letter is the middle position (index 3)
    for c in &all_chars {
        if c != &solution[0] && c != &solution[6] {
            if five_length_1.contains(&c) && five_length_2.contains(&c) && five_length_3.contains(&c) {
                solution[3] = c.clone();
            } 
        }
    }

    // Letter in 4 which is not in 1 which is not the middle position letter is the top left position (index 1)
    for c in &four {
        if c != &solution[3] {
            if !one.contains(&c) {
                solution[1] = c.clone();
            }
        }
    }

    // Letter which is in only one of the five character strings (2, 3, and 5),
    // which is not the top, middle, or bottom position letter (index 0, index 3, or index 6)
    // which is not the top left position letter (index 1)
    // is the bottom left position (index 4)
    for c in &all_chars {
        if c != &solution[0] && c != &solution[1] && c != &solution[3] && c != &solution[6] {
            if (five_length_1.contains(&c) && !five_length_2.contains(&c) && !five_length_3.contains(&c))
                || (!five_length_1.contains(&c) && five_length_2.contains(&c) && !five_length_3.contains(&c)) 
                || (!five_length_1.contains(&c) && !five_length_2.contains(&c) && five_length_3.contains(&c)) {
                    solution[4] = c.clone();
                }
        }
    }

    // Letter which is in 1
    // which is not in one of the six character strings (0, 6, 9)
    // which is not any of the previously found characters (indexes 0, 1, 3, 4, 6)
    // is the top right position (index 2)
    for c in &all_chars {
        if one.contains(&c) {
            if !solution.contains(&c) {
                if (six_length_1.contains(&c) && six_length_2.contains(&c) && !six_length_3.contains(&c))
                    || (six_length_1.contains(&c) && !six_length_2.contains(&c) && six_length_3.contains(&c))
                    || (!six_length_1.contains(&c) && six_length_2.contains(&c) && six_length_3.contains(&c)) {
                        solution[2] = c.clone();
                    }
            }
        }
    }

    // Letter not in solution at this point is the final letter in the bottom right position (index 5)
    for c in ['a','b','c','d','e','f','g'] {
        if !solution.contains(&c) {
            solution[5] = c.clone();
        }
    }

    let mut final_zero = [solution[0], solution[1], solution[2], solution[4], solution[5], solution[6]];
    final_zero.sort_by(|a, b| a.cmp(b));
    let mut final_zero_string: String = final_zero.into_iter().collect();
    let mut final_one = [solution[2], solution[5]];
    final_one.sort_by(|a, b| a.cmp(b));
    let mut final_one_string: String = final_one.into_iter().collect();
    let mut final_two = [solution[0], solution[2], solution[3], solution[4], solution[6]];
    final_two.sort_by(|a, b| a.cmp(b));
    let mut final_two_string: String = final_two.into_iter().collect();
    let mut final_three = [solution[0], solution[2], solution[3], solution[5], solution[6]];
    final_three.sort_by(|a, b| a.cmp(b));
    let mut final_three_string: String = final_three.into_iter().collect();
    let mut final_four = [solution[1], solution[2], solution[3], solution[5]];
    final_four.sort_by(|a, b| a.cmp(b));
    let mut final_four_string: String = final_four.into_iter().collect();
    let mut final_five = [solution[0], solution[1], solution[3], solution[5], solution[6]];
    final_five.sort_by(|a, b| a.cmp(b));
    let mut final_five_string: String = final_five.into_iter().collect();
    let mut final_six = [solution[0], solution[1], solution[3], solution[4], solution[5], solution[6]];
    final_six.sort_by(|a, b| a.cmp(b));
    let mut final_six_string: String = final_six.into_iter().collect();
    let mut final_seven = [solution[0], solution[2], solution[5]];
    final_seven.sort_by(|a, b| a.cmp(b));
    let mut final_seven_string: String = final_seven.into_iter().collect();
    let mut final_eight = [solution[0], solution[1], solution[2], solution[3], solution[4], solution[5], solution[6]];
    final_eight.sort_by(|a, b| a.cmp(b));
    let mut final_eight_string: String = final_eight.into_iter().collect();
    let mut final_nine = [solution[0], solution[1], solution[2], solution[3], solution[5], solution[6]];
    final_nine.sort_by(|a, b| a.cmp(b));
    let mut final_nine_string: String = final_nine.into_iter().collect();

    // println!("{:?}", solution);

    let mut number = Vec::<char>::new();
    for output in outputs {
        let mut o: Vec<char> = output.chars().collect();
        o.sort_by(|a, b| a.cmp(b));
        let o_sorted: String = o.into_iter().collect();

        if o_sorted == final_zero_string {
            number.push('0');
        } else if o_sorted == final_one_string {
            number.push('1');
        } else if o_sorted == final_two_string {
            number.push('2');
        } else if o_sorted == final_three_string {
            number.push('3');
        } else if o_sorted == final_four_string {
            number.push('4');
        } else if o_sorted == final_five_string {
            number.push('5');
        } else if o_sorted == final_six_string {
            number.push('6');
        } else if o_sorted == final_seven_string {
            number.push('7');
        } else if o_sorted == final_eight_string {
            number.push('8');
        } else if o_sorted == final_nine_string {
            number.push('9');
        }
    }

    let final_number_string: String = number.into_iter().collect();
    let final_number = final_number_string.parse::<i32>().unwrap();

    println!("{}", final_number);

    final_number
}
