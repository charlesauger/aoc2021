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
