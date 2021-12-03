use crate::helpers::read_lines;

fn parse_binary_string(binary_string: &str) -> i32 {
    let mut result = 0;
    let mut column = 1;
    
    for c in binary_string.chars().rev() {
        match c {
            '1' => result += column,
            _ => ()
        }

        column *= 2;
    }
    
    result
}



pub fn day_3_part_1() {
    let mut ones: [i32; 12] = [0; 12];
    let mut total_lines = 0;

    if let Ok(lines) = read_lines("resources/day_3.txt") {
        for line_result in lines {
            if let Ok(line) = line_result {
                
                for (i, c) in line.chars().enumerate() {
                    match c {
                        '1' => ones[i] += 1,
                        _ => ()
                    }
                }

                total_lines += 1;
            }
        }

        let mut gamma_string = String::new();
        let mut epsilon_string = String::new();

        println!("Total lines: {}", total_lines);

        let mut test = 0;
        for i in ones {
            println!("ones[{}] = {}", test, ones[test]);
            if i > total_lines / 2 {
                gamma_string.push('1');
                epsilon_string.push('0');
            } else {
                gamma_string.push('0');
                epsilon_string.push('1');
            }
            test += 1;
        }

        let gamma = parse_binary_string(&gamma_string);
        let epsilon = parse_binary_string(&epsilon_string);

        println!("Gamma: {} = {}", gamma_string, gamma);
        println!("Epsilon: {} = {}", epsilon_string, epsilon);
        println!("Result: {}", gamma * epsilon);
    }
}
