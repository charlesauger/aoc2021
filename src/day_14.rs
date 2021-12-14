use crate::helpers::read_lines;
use std::collections::HashMap;

pub fn day_14_part_1() {
    if let Ok(file_lines) = read_lines("resources/day_14.txt") {
        let mut polymer = Vec::<char>::new();

        let mut rules = HashMap::<(char, char), char>::new();

        let mut first = true;

        for file_line in file_lines {
            let line = file_line.unwrap();
        
            if first {
                for c in line.chars() {
                    polymer.push(c);
                }

                first = false;
            } else {
                // Skip the empty line
                if line.len() == 0 {
                    continue;
                }

                let mut split = line.split("->");

                let rule = split.next().unwrap().trim();
                let to_add = split.next().unwrap().trim();

                let mut first_rule_char: char = ' ';
                let mut second_rule_char: char = ' ';
                let mut to_add_char: char = ' ';

                let mut first_char = true;
                for c in rule.chars() {
                    if first_char {
                        first_rule_char = c;
                        first_char = false;
                    } else {
                        second_rule_char = c;
                    }
                }

                for c in to_add.chars() {
                    to_add_char = c;
                }

                rules.insert((first_rule_char, second_rule_char), to_add_char);
            }
        }

        for i in 0..polymer.len() {
            print!("{}", polymer.get(i).unwrap());
        }

        println!();
        println!("Rules = {:?}", rules);

        for step in 1..=10 {
            let mut to_insert = Vec::<(usize, char)>::new();
            let mut insert_pos = 1;
            for i in 0..(polymer.len()-1) {
                let first_char = polymer.get(i).unwrap();
                let second_char = polymer.get(i+1).unwrap();
                let char_to_insert = rules.get(&(*first_char, *second_char)).unwrap();
    
                to_insert.push((insert_pos, *char_to_insert));
    
                insert_pos += 2;
            }
    
            for insert in to_insert {
                polymer.insert(insert.0, insert.1);
            }
    
            // print!("After step {}:", step);
            // for i in 0..polymer.len() {
            //     print!("{}", polymer.get(i).unwrap());
            // }
            // println!();

            println!("Step {} completed", step);
        }

        let letters = vec!['V', 'N', 'K', 'S', 'F', 'P', 'B', 'C', 'H'];
        let mut counts = HashMap::<char, usize>::new();

        for c in letters {
            counts.insert(c, polymer.iter().filter(|character| **character == c).count());
        }

        println!("{:?}", counts);

        let mut lowest_char = 'V';
        let mut lowest_count = usize::MAX;

        let mut highest_char = 'V';
        let mut highest_count = usize::MIN;

        for el in counts {
            if el.1 < lowest_count {
                lowest_char = el.0;
                lowest_count = el.1;
            }

            if el.1 > highest_count {
                highest_char = el.0;
                highest_count = el.1;
            }
        }

        println!("Lowest char {} count = {}", lowest_char, lowest_count);
        println!("Highest char {} count = {}", highest_char, highest_count);

    }
}
