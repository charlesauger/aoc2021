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
    if let Ok(file_lines) = read_lines("resources/day_14.txt") {
        let mut polymer = String::new();

        let mut rules = HashMap::<String, (char, String, String)>::new();
        let mut char_counts = HashMap::<char, usize>::new();
        let mut counts = HashMap::<String, usize>::new();

        let mut first = true;

        let mut letters = HashSet::<char>::new();

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

                letters.insert(first_rule_char);
                letters.insert(second_rule_char);
                letters.insert(to_add_char);

                let mut first_result = first_rule_char.to_string();
                first_result.push(to_add_char);

                let mut second_result = to_add_char.to_string();
                second_result.push(second_rule_char);

                counts.insert(first_result.clone(), 0);
                counts.insert(second_result.clone(), 0);

                rules.insert(rule, (to_add_char, first_result, second_result));
            }
        }

        // println!("{}", polymer);

        // println!("Rules = {:?}", rules);

        for c in polymer.chars() {
            if char_counts.contains_key(&c) {
                char_counts.insert(c, char_counts.get(&c).unwrap() + 1);
            } else {
                char_counts.insert(c, 1);
            }
        }

        for c in letters {
            if !char_counts.contains_key(&c) {
                char_counts.insert(c, 0);
            }
        }

        for i in 0..(polymer.len()-1) {
            let first_char = polymer.chars().nth(i).unwrap();
            let second_char = polymer.chars().nth(i+1).unwrap();

            let mut concat_string = first_char.to_string();
            concat_string.push(second_char);

            let curr_number = counts.get(&concat_string);
            if curr_number.is_none() {
                counts.insert(concat_string, 1);
            } else {
                counts.insert(concat_string, curr_number.unwrap() + 1);
            }
        }

        // println!("{:?}", counts);
        // println!("{:?}", char_counts);

        for i in 1..=40 {
            let mut to_remove = Vec::<(String, usize)>::new();
            let mut to_add = Vec::<(String, usize)>::new();
            for count in &counts {
                let rule = rules.get(count.0).unwrap();

                let char_rule_result = rule.0;
                let first_rule_result = &rule.1;
                let second_rule_result = &rule.2;

                let input_string = count.0;
                let number_of_times = count.1;

                // We need to
                // 1. Remove all of the current instances of input_string
                // 2. Add the char in the rule to the char_counts
                // 3. Add the two strings in the rule to the counts

                let curr_char_count = char_counts.get(&char_rule_result).unwrap().clone();
                let curr_first_rule_result_count = counts.get(first_rule_result).unwrap();
                let curr_second_rule_result_count = counts.get(second_rule_result).unwrap();

                // println!("Adding {} {} times", char_rule_result, number_of_times);

                char_counts.insert(char_rule_result, curr_char_count + number_of_times);

                to_remove.push((input_string.clone(), number_of_times.clone()));

                to_add.push((first_rule_result.clone(), number_of_times.clone()));
                to_add.push((second_rule_result.clone(), number_of_times.clone()));
            }

            for add in to_add {
                let curr_amount = &counts.get(&add.0).unwrap().clone();
                counts.insert(add.0, curr_amount + add.1);
            }

            for remove in to_remove {
                let curr_amount = &counts.get(&remove.0).unwrap().clone();
                counts.insert(remove.0, curr_amount - remove.1);
            }
        }

        // println!("{:?}", counts);
        // println!("{:?}", char_counts);

        let mut result_vector = Vec::<(usize, char)>::new();
        for el in char_counts {
            result_vector.push((el.1, el.0));
        }

        result_vector.sort();

        // for x in &result_vector {
        //     println!("{:?}", x);
        // }

        // let lowest = &result_vector.get(0).unwrap().0;
        let mut lowest = usize::MAX;
        for x in &result_vector {
            if x.0 != 0 {
                lowest = x.0;
                break;
            }
        }
        let highest = &result_vector.get(result_vector.len()-1).unwrap().0;

        println!("Highest - lowest = {} - {} = {}", highest, lowest, highest - lowest);

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
