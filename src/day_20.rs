use crate::helpers::read_lines;

pub fn day_20_part_1() {
    let mut iea = vec![false; 512];

    if let Ok(file_lines) = read_lines("resources/day_20_test.txt") {

        let mut first = true;
        for file_line in file_lines {
            let line = file_line.unwrap();

            if file_line.len() == 0 {
                continue;
            }

            if first {
                for c in file_line.chars() {
                    
                }
            }
        }
    }
}
