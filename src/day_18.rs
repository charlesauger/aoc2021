use crate::helpers::read_lines;

// fn print_snailfish(snailfish: &Vec<(i32, i32)>) {
//     let initial_depth = snailfish[0].0;

//     for i in 0..initial_depth {
//         print!("[");
//     }

//     let mut curr_depth = initial_depth;

//     for i in 0..snailfish.len() {
//         if snailfish[i].0 < curr_depth {
//             print!("]")
//         }

//         if i != 0 {
//             print!(",");
//         }

//         if snailfish[i].0 > curr_depth {
//             print!("[");
//         }

//         print!("{}", snailfish[i].1);

//         curr_depth = snailfish[i].0;
//     }

//     println!();
// }

pub fn day_18_part_1() {
    if let Ok(file_lines) = read_lines("resources/day_18.txt") {

        let mut snailfish_list = Vec::<Vec<(i32, i32)>>::new();
        
        for file_line in file_lines {
            let line = file_line.unwrap();

            println!("{}", line);

            let mut snailfish = Vec::<(i32, i32)>::new();
            let mut depth = 0;

            for c in line.chars() {
                if c == '[' {
                    depth += 1;
                } else if c == ']' {
                    depth -= 1;
                } else if c == ',' {
                    // Do nothing?
                } else if ['0','1','2','3','4','5','6','7','8','9'].contains(&c) {
                    // c is a number
                    snailfish.push((depth, c.to_digit(10).unwrap() as i32));
                } else {
                    // Do nothing
                }
            }

            println!("{:?}", snailfish);

            snailfish_list.push(snailfish);
        }

        let mut added_snailfish = Vec::<(i32, i32)>::new();
        added_snailfish.append(&mut snailfish_list[0]);
        let mut pointer: usize = 1;

        loop {
            added_snailfish.append(&mut snailfish_list[pointer]);

            for i in 0..added_snailfish.len() {
                added_snailfish[i].0 += 1;
            }

            println!("Added snailfish before reduction = {:?}", added_snailfish);

            loop {
                let mut reduced = false;
    
                let mut curr_depth = 0;
                let mut num_at_same_depth = 0;
    
                for i in 0..added_snailfish.len() {
                    let i_depth = added_snailfish[i].0;
    
                    if i_depth != curr_depth {
                        num_at_same_depth = 1;
                    } else {
                        num_at_same_depth += 1;
                    }
    
                    curr_depth = i_depth;
    
                    if i_depth > 4 {
                        // find if number is part of a pair
    
                        if (i+1) < added_snailfish.len() {
                            // if it's the leftmost number and the number to the right is the same depth
                            if i == 0 && (added_snailfish[i+1].0 == i_depth) {
                                // EXPLODE
                                reduced = true;
                            }
    
                            if num_at_same_depth % 2 != 0 {
                                // Could be the first of a pair, check to right
                                if added_snailfish[i+1].0 == i_depth {
                                    // EXPLODE
                                    reduced = true;
                                }
                            }
    
                            if reduced {
                                // EXPLODE FOR REAL HERE
                                if i > 0 {
                                    added_snailfish[i-1].1 += added_snailfish[i].1;
                                }
    
                                if i + 2 < added_snailfish.len() {
                                    added_snailfish[i+2].1 += added_snailfish[i+1].1;
                                }
    
                                added_snailfish[i].0 -= 1;
                                added_snailfish[i].1 = 0;
    
                                added_snailfish.remove(i+1);

                                // println!("After explode {:?}", added_snailfish);
    
                                break;
                            }
                        }
                    }
                }

                if reduced {
                    continue;
                }

                for i in 0..added_snailfish.len() {
                    if added_snailfish[i].1 >= 10 {
                        // split number
                        let left = ((added_snailfish[i].1) as f32 / 2.0).floor() as i32;
                        let right = ((added_snailfish[i].1) as f32 / 2.0).ceil() as i32;
    
                        added_snailfish[i].0 += 1;
                        added_snailfish[i].1 = left;
    
                        added_snailfish.insert(i+1, (added_snailfish[i].0, right));
    
                        reduced = true;
    
                        // println!("After split {:?}", added_snailfish);
                        break;
                    }
                }
    
                if !reduced {
                    break;
                }
            }
    
            println!("------------------");
            println!("Reduced added_snailfish = {:?}", added_snailfish);
            println!("Just the numbers: ");
            for i in 0..added_snailfish.len() {
                print!("{}, ", added_snailfish[i].1);
            }
            println!();
            println!("------------------");

            pointer += 1;
            if pointer == snailfish_list.len() {
                break;
            }
        }

        println!("Final reduced snailfish = {:?}", added_snailfish);

        

        println!("Final magnitude = {}", calculate_magnitude(&added_snailfish));
    }
}

pub fn day_18_part_2() {
    if let Ok(file_lines) = read_lines("resources/day_18.txt") {

        let mut snailfish_list = Vec::<Vec<(i32, i32)>>::new();
        
        for file_line in file_lines {
            let line = file_line.unwrap();

            println!("{}", line);

            let mut snailfish = Vec::<(i32, i32)>::new();
            let mut depth = 0;

            for c in line.chars() {
                if c == '[' {
                    depth += 1;
                } else if c == ']' {
                    depth -= 1;
                } else if c == ',' {
                    // Do nothing?
                } else if ['0','1','2','3','4','5','6','7','8','9'].contains(&c) {
                    // c is a number
                    snailfish.push((depth, c.to_digit(10).unwrap() as i32));
                } else {
                    // Do nothing
                }
            }

            println!("{:?}", snailfish);

            snailfish_list.push(snailfish);
        }

        let mut highest_magnitude = i32::MIN;

        for i in 0..snailfish_list.len() {
            for j in 0..snailfish_list.len() {

                if i == j {
                    continue;
                }

                let mut added_snailfish_first = Vec::<(i32, i32)>::new();

                added_snailfish_first.append(&mut snailfish_list[i].clone());
                added_snailfish_first.append(&mut snailfish_list[j].clone());
                for i in 0..added_snailfish_first.len() {
                    added_snailfish_first[i].0 += 1;
                }

                loop {
                    let mut reduced = false;
        
                    let mut curr_depth = 0;
                    let mut num_at_same_depth = 0;
        
                    for i in 0..added_snailfish_first.len() {
                        let i_depth = added_snailfish_first[i].0;
        
                        if i_depth != curr_depth {
                            num_at_same_depth = 1;
                        } else {
                            num_at_same_depth += 1;
                        }
        
                        curr_depth = i_depth;
        
                        if i_depth > 4 {
                            // find if number is part of a pair
        
                            if (i+1) < added_snailfish_first.len() {
                                // if it's the leftmost number and the number to the right is the same depth
                                if i == 0 && (added_snailfish_first[i+1].0 == i_depth) {
                                    // EXPLODE
                                    reduced = true;
                                }
        
                                if num_at_same_depth % 2 != 0 {
                                    // Could be the first of a pair, check to right
                                    if added_snailfish_first[i+1].0 == i_depth {
                                        // EXPLODE
                                        reduced = true;
                                    }
                                }
        
                                if reduced {
                                    // EXPLODE FOR REAL HERE
                                    if i > 0 {
                                        added_snailfish_first[i-1].1 += added_snailfish_first[i].1;
                                    }
        
                                    if i + 2 < added_snailfish_first.len() {
                                        added_snailfish_first[i+2].1 += added_snailfish_first[i+1].1;
                                    }
        
                                    added_snailfish_first[i].0 -= 1;
                                    added_snailfish_first[i].1 = 0;
        
                                    added_snailfish_first.remove(i+1);
    
                                    // println!("After explode {:?}", added_snailfish);
        
                                    break;
                                }
                            }
                        }
                    }
    
                    if reduced {
                        continue;
                    }
    
                    for i in 0..added_snailfish_first.len() {
                        if added_snailfish_first[i].1 >= 10 {
                            // split number
                            let left = ((added_snailfish_first[i].1) as f32 / 2.0).floor() as i32;
                            let right = ((added_snailfish_first[i].1) as f32 / 2.0).ceil() as i32;
        
                            added_snailfish_first[i].0 += 1;
                            added_snailfish_first[i].1 = left;
        
                            added_snailfish_first.insert(i+1, (added_snailfish_first[i].0, right));
        
                            reduced = true;
        
                            // println!("After split {:?}", added_snailfish_first);
                            break;
                        }
                    }
        
                    if !reduced {
                        break;
                    }
                }

                // println!("Calculating first magnitude");
                // magnitudes.push(calculate_magnitude_fixed(&mut added_snailfish_first));
                let first_magnitude = calculate_magnitude_fixed(&mut added_snailfish_first);
                if first_magnitude > highest_magnitude {
                    highest_magnitude = first_magnitude;
                }


                let mut added_snailfish_second = Vec::<(i32, i32)>::new();

                added_snailfish_second.append(&mut snailfish_list[j].clone());
                added_snailfish_second.append(&mut snailfish_list[i].clone());
                for i in 0..added_snailfish_second.len() {
                    added_snailfish_second[i].0 += 1;
                }

                loop {
                    let mut reduced = false;
        
                    let mut curr_depth = 0;
                    let mut num_at_same_depth = 0;
        
                    for i in 0..added_snailfish_second.len() {
                        let i_depth = added_snailfish_second[i].0;
        
                        if i_depth != curr_depth {
                            num_at_same_depth = 1;
                        } else {
                            num_at_same_depth += 1;
                        }
        
                        curr_depth = i_depth;
        
                        if i_depth > 4 {
                            // find if number is part of a pair
        
                            if (i+1) < added_snailfish_second.len() {
                                // if it's the leftmost number and the number to the right is the same depth
                                if i == 0 && (added_snailfish_second[i+1].0 == i_depth) {
                                    // EXPLODE
                                    reduced = true;
                                }
        
                                if num_at_same_depth % 2 != 0 {
                                    // Could be the first of a pair, check to right
                                    if added_snailfish_second[i+1].0 == i_depth {
                                        // EXPLODE
                                        reduced = true;
                                    }
                                }
        
                                if reduced {
                                    // EXPLODE FOR REAL HERE
                                    if i > 0 {
                                        added_snailfish_second[i-1].1 += added_snailfish_second[i].1;
                                    }
        
                                    if i + 2 < added_snailfish_second.len() {
                                        added_snailfish_second[i+2].1 += added_snailfish_second[i+1].1;
                                    }
        
                                    added_snailfish_second[i].0 -= 1;
                                    added_snailfish_second[i].1 = 0;
        
                                    added_snailfish_second.remove(i+1);
        
                                    break;
                                }
                            }
                        }
                    }
    
                    if reduced {
                        continue;
                    }
    
                    for i in 0..added_snailfish_second.len() {
                        if added_snailfish_second[i].1 >= 10 {
                            // split number
                            let left = ((added_snailfish_second[i].1) as f32 / 2.0).floor() as i32;
                            let right = ((added_snailfish_second[i].1) as f32 / 2.0).ceil() as i32;
        
                            added_snailfish_second[i].0 += 1;
                            added_snailfish_second[i].1 = left;
        
                            added_snailfish_second.insert(i+1, (added_snailfish_second[i].0, right));
        
                            reduced = true;
        
                            break;
                        }
                    }
        
                    if !reduced {
                        break;
                    }
                }

                // println!("Calculating second magnitude");
                // magnitudes.push(calculate_magnitude_fixed(&mut added_snailfish_second));
                
                let second_magnitude = calculate_magnitude_fixed(&mut added_snailfish_second);
                if second_magnitude > highest_magnitude {
                    highest_magnitude = second_magnitude;
                }


            }
        }

        // println!("{:?}", magnitudes);
        // let max = magnitudes.iter().max().unwrap();
        // println!("Maximum magnitude = {}", max);
        println!("Maximum magnitude = {}", highest_magnitude);

        
    }
}

fn calculate_magnitude_fixed(snailfish: &mut Vec<(i32, i32)>) -> i32 {
    loop {
        if snailfish.len() == 1 {
            return snailfish[0].1;
        } 

        for i in 0..(snailfish.len()-1) {
            if snailfish[i].0 == snailfish[i+1].0 {
                snailfish[i].1 = snailfish[i].1 * 3 + snailfish[i+1].1 * 2;
                snailfish[i].0 -= 1;

                snailfish.remove(i+1);
                break;
            }
        }
    }
}

fn calculate_magnitude(snailfish: &Vec<(i32, i32)>) -> i32 {
    let mut num_list = Vec::<i32>::new();
    for i in 0..snailfish.len() {
        num_list.push(snailfish[i].1);
    }

    loop {
        let mut new_num_list = Vec::<i32>::new();
        let mut i = 0;
        while i < num_list.len() {
            new_num_list.push(num_list[i] * 3 + num_list[i+1] * 2);
            i += 2;
        }

        num_list = new_num_list.clone();

        if num_list.len() == 1 {
            break;
        }
    }

    num_list[0]
}
