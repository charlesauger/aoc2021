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

        for i in ones {
            if i > total_lines / 2 {
                gamma_string.push('1');
                epsilon_string.push('0');
            } else {
                gamma_string.push('0');
                epsilon_string.push('1');
            }
        }

        let gamma = parse_binary_string(&gamma_string);
        let epsilon = parse_binary_string(&epsilon_string);

        println!("Gamma: {} = {}", gamma_string, gamma);
        println!("Epsilon: {} = {}", epsilon_string, epsilon);
        println!("Result: {}", gamma * epsilon);
    }
}

pub fn day_3_part_2() {
    let mut strings = Vec::<String>::new();

    if let Ok(lines) = read_lines("resources/day_3.txt") {
    // if let Ok(lines) = read_lines("resources/day_3_test.txt") {
        for line_result in lines {
            if let Ok(line) = line_result {
                strings.push(line);
            }
        }
    }

    let mut strings_1 = strings.clone();
    let mut strings_2 = strings.clone();

    println!("{:?}", strings_1);

    for i in 0..12 {
        if strings_1.len() == 1 {
            break;
        }

        let ones = strings_1.iter().filter(|&n| n.chars().nth(i).unwrap() == '1').count();
        println!("Ones = {}", ones);

        if ones >= (strings_1.len() - ones) {
            println!("Retaining strings with 1 in {} position", i);
            strings_1.retain(|value| value.chars().nth(i).unwrap() == '1');
        } else {
            println!("Retaining strings with 0 in {} position", i);
            strings_1.retain(|value| value.chars().nth(i).unwrap() == '0');
        }
        println!("{:?}", strings_1);
        println!("Size of strings_1 = {}", strings_1.len());
    }

    println!("{:?}", strings_2);

    for i in 0..12 {
        if strings_2.len() == 1 {
            break;
        }

        let ones = strings_2.iter().filter(|&n| n.chars().nth(i).unwrap() == '1').count();
        println!("Ones = {}", ones);

        if ones >= (strings_2.len() - ones) {
            println!("Retaining strings with 1 in {} position", i);
            strings_2.retain(|value| value.chars().nth(i).unwrap() == '0');
        } else {
            println!("Retaining strings with 0 in {} position", i);
            strings_2.retain(|value| value.chars().nth(i).unwrap() == '1');
        }
        println!("{:?}", strings_2);
        println!("Size of strings_2 = {}", strings_2.len());
    }

    assert!(strings_1.len() == 1);
    assert!(strings_2.len() == 1);

    let oxygen_binary = strings_1.first().unwrap();
    let c02_binary = strings_2.first().unwrap();

    println!("oxygen binary: {}", oxygen_binary);
    println!("c02 binary: {}", c02_binary);

    let oxygen_generator_rating = parse_binary_string(oxygen_binary);
    let c02_scrubber_rating = parse_binary_string(c02_binary);

    println!("Oxygen generator rating: {}", oxygen_generator_rating);
    println!("C02 scrubber rating: {}", c02_scrubber_rating);
    println!("Result: {}", oxygen_generator_rating * c02_scrubber_rating);
}
