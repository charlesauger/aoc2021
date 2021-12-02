use crate::helpers::read_lines;

pub fn day_2_part_1() {
    let mut horizontal_position = 0;
    let mut depth = 0;

    if let Ok(lines) = read_lines("resources/day_2.txt") {
        for line_result in lines {
            if let Ok(line) = line_result {
                let split = line.split_whitespace().collect::<Vec<_>>();
    
                let direction = split[0];
                let distance: i32 = split[1].parse().unwrap();

                match direction {
                    "forward" => horizontal_position += distance,
                    "down" => depth += distance,
                    "up" => depth -= distance,
                    _ => ()
                }
            }
        }
    }

    println!("Horizontal position = {}", horizontal_position);
    println!("Depth = {}", depth);
    println!("Horizontal position * Depth = {}", horizontal_position * depth);
}
