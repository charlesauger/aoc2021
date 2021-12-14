use crate::helpers::read_lines;
use std::collections::{HashMap, HashSet};

pub fn day_14_part_1() {
    if let Ok(file_lines) = read_lines("resources/day_14_test_2.txt") {
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

        for step in 1..=40 {
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

pub fn day_14_part_2() {
    if let Ok(file_lines) = read_lines("resources/day_14_test.txt") {
        let mut polymer = String::new();

        let mut rules = HashMap::<String, (String, String)>::new();
        let mut char_counts = HashMap::<char, usize>::new();
        let mut counts = HashMap::<String, usize>::new();

        let mut first = true;

        for file_line in file_lines {
            let line = file_line.unwrap();
        
            if first {
                polymer = line;

                first = false;
            } else {
                // Skip the empty line
                if line.len() == 0 {
                    continue;
                }

                let mut split = line.split("->");

                let rule = split.next().unwrap().trim().to_string();
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

                let mut first_result = first_rule_char.to_string();
                first_result.push(to_add_char);

                let mut second_result = to_add_char.to_string();
                second_result.push(second_rule_char);

                rules.insert(rule, (first_result, second_result));
            }
        }

        println!("{}", polymer);

        println!("Rules = {:?}", rules);

        let letters = vec!['V','N','K','S','F','P','B','S','C','H','O'];

        for c in polymer.chars() {
            if char_counts.contains_key(&c) {
                char_counts.insert(c, char_counts.get(&c).unwrap() + 1);
            } else {
                char_counts.insert(c, 1);
            }
        }

        for i in 0..(polymer.len()-1) {
            let first_char = polymer.chars().nth(i).unwrap();
            let second_char = polymer.chars().nth(i+1).unwrap();

            let mut concat_string = first_char.to_string();
            concat_string.push(second_char);

            counts.insert(concat_string, 1);
        }

        println!("{:?}", counts);

        for _i in 1..=40 {
            let mut chars_to_add = HashMap::<char, usize>::new();
            let mut to_add = HashMap::<String, usize>::new();
            let mut to_remove = HashSet::<String>::new();
            for el in counts.iter_mut() {
                let result = rules.get(el.0).unwrap();

                let first_result = result.0.clone();
                let second_result = result.1.clone();

                if to_add.contains_key(&first_result) {
                    let mut current_count: usize = to_add.get(&first_result).unwrap().clone();
                    current_count += el.1.clone();
                    to_add.insert(first_result, current_count);
                } else {
                    let mut current_count: usize = 0;
                    current_count += el.1.clone();
                    to_add.insert(first_result, current_count);
                }

                if to_add.contains_key(&second_result) {
                    let mut current_count: usize = to_add.get(&second_result).unwrap().clone();
                    current_count += el.1.clone();
                    to_add.insert(second_result, current_count);
                } else {
                    let mut current_count: usize = 0;
                    current_count += el.1.clone();
                    to_add.insert(second_result, current_count);
                }

                // Get the first char of the second result, this is the character being added to the whole string
                let char_to_add = result.1.clone().chars().next().unwrap();
                if char_counts.contains_key(&char_to_add) {
                    char_counts.insert(char_to_add, char_counts.get(&char_to_add).unwrap() + el.1.clone());
                } else {
                    char_counts.insert(char_to_add, el.1.clone());
                }

                to_remove.insert(el.0.clone());
            }

            for el in to_add {
                if counts.contains_key(&el.0) {
                    let mut current_count: usize = *counts.get(&el.0).unwrap();
                    counts.insert(el.0, current_count + el.1);
                } else {
                    counts.insert(el.0, el.1);
                }
            }
        }

        println!("{:?}", counts);
        println!("{:?}", char_counts);



        // for i in 1..2 {
        //     for el in &rules {
        //         let curr_key = (*el.0, i as usize);
                
        //         let map = HashMap::<char, usize>::new();
        //         for l in letters {
        //             map.insert(l, 0);
        //         }

        //         if i == 1 {
        //             map.insert(*rules.get(&(curr_key.0.0, curr_key.0.1)).unwrap(), 1);

        //             memo.insert(curr_key, map);
        //         }
        //         else {

        //         }
        //     }
        // }

    }
}
