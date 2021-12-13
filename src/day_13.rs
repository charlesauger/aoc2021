use crate::helpers::read_lines;
use std::collections::HashSet;

pub fn day_13_part_1() {
    if let Ok(file_lines) = read_lines("resources/day_13.txt") {
        
        // day_13_test
        // Largest x = 10 (+1)
        // Largest y = 14 (+1)

        // day_13
        // Largest x = 1310 (+1)
        // Largest y = 893 (+1)

        let mut dot_set = HashSet::<(i32, i32)>::new();

        let mut curr_width = 1311;
        let mut curr_height = 894;
        // let mut curr_width = 11;
        // let mut curr_height = 15;

        let mut finding_dots = true;
        for file_line in file_lines {
            let line = file_line.unwrap();

            if line.len() == 0 {
                finding_dots = false;
                continue;
            }

            if finding_dots {
                // Find and fill in the dots in the array
                let mut split = line.split(",");

                let x_pos = split.next().unwrap().parse::<i32>().unwrap();
                let y_pos = split.next().unwrap().parse::<i32>().unwrap();

                dot_set.insert((x_pos, y_pos));
            } else {

                // Fold the array along certain lines
                let mut first_split = line.split(" ");
                first_split.next();
                first_split.next();

                let mut fold_string = first_split.next().unwrap().split("=");

                let side = fold_string.next().unwrap();
                let position = fold_string.next().unwrap().parse::<i32>().unwrap();

                let mut to_remove = Vec::<(i32, i32)>::new();
                let mut to_add = Vec::<(i32, i32)>::new();
                
                if side == "x" {
                    let half_width: i32 = curr_width / 2;
                    // let half_width = position;
                    assert!(position == half_width);

                    for dot_pos in &dot_set {
                        if dot_pos.0 > half_width {
                            let folded_x_pos = (dot_pos.0-(curr_width-1)).abs();
                            to_add.push((folded_x_pos, dot_pos.1));
                            to_remove.push((dot_pos.0, dot_pos.1));
                        }
                    }
                    curr_width /= 2;
                } else if side == "y" {
                    let half_height: i32 = curr_height / 2;
                    // let half_height = position;
                    assert!(position == half_height);

                    for dot_pos in &dot_set {
                        if dot_pos.1 > half_height {
                            let folded_y_pos = (dot_pos.1-(curr_height-1)).abs();
                            to_add.push((dot_pos.0, folded_y_pos));
                            to_remove.push((dot_pos.0, dot_pos.1));
                        }
                    }
                    curr_height /= 2;
                }
                
                // println!("to_remove size = {}", to_remove.len());
                
                // println!("to_add size = {}", to_add.len());

                for pos in to_remove {
                    // println!("Removing dot at pos {},{}", pos.0, pos.1);
                    dot_set.remove(&pos);
                }

                for pos in to_add {
                    // println!("Adding dot at pos {},{}", pos.0, pos.1);
                    dot_set.insert(pos);
                }
            }
        }

        for i in 0..8 {
            for j in 0..40 {
                if dot_set.contains(&(j, i)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        let dot_count = dot_set.len();
        
        println!("Number of dots = {}", dot_count);

        // println!("Largest x = {} - largest y = {}", largest_x, largest_y);
    }
}

// pub fn day_13_part_1() {
//     if let Ok(file_lines) = read_lines("resources/day_13_test.txt") {
        
//         // day_13_test
//         // Largest x = 10 (+1)
//         // Largest y = 14 (+1)

//         // day_13
//         // Largest x = 1310 (+1)
//         // Largest y = 893 (+1)

//         const WIDTH: i32 = 20;
//         const HEIGHT: i32 = 20;

//         let mut curr_width = 11;
//         let mut curr_height = 15;

//         let mut dots = [[false; HEIGHT as usize]; WIDTH as usize];

//         let mut finding_dots = true;
//         for file_line in file_lines {
//             let line = file_line.unwrap();

//             if line.len() == 0 {
//                 finding_dots = false;
//                 continue;
//             }

//             if finding_dots {
//                 // Find and fill in the dots in the array
//                 let mut split = line.split(",");

//                 let x_pos = split.next().unwrap().parse::<i32>().unwrap();
//                 let y_pos = split.next().unwrap().parse::<i32>().unwrap();

//                 // if x_pos > largest_x {
//                 //     largest_x = x_pos;
//                 // }

//                 // if y_pos > largest_y {
//                 //     largest_y = y_pos;
//                 // }

//                 dots[x_pos as usize][y_pos as usize] = true;
//             } else {
//                 // println!("BEFORE FOLDING");
//                 // for i in 0..20 {
//                 //     for j in 0..20 {
//                 //         if dots[j][i] {
//                 //             print!("#");
//                 //         } else {
//                 //             print!(".");
//                 //         }
//                 //     }
//                 //     println!();
//                 // }

//                 // Fold the array along certain lines
//                 let mut first_split = line.split(" ");
//                 first_split.next();
//                 first_split.next();

//                 let mut fold_string = first_split.next().unwrap().split("=");

//                 let side = fold_string.next().unwrap();
//                 let position = fold_string.next().unwrap().parse::<i32>().unwrap();

//                 if side == "x" {
//                     let half_width: i32 = curr_width / 2;

//                     for x in half_width..curr_width {
//                         for y in 0..curr_height {
//                             if dots[x as usize][y as usize] {
//                                 println!("Dot at position {},{}", x, y);
//                                 println!("Folding to put dot at position {},{}", (x-(curr_width-1)).abs(), y);
//                                 dots[(x-(curr_width-1)).abs() as usize][y as usize] = true;
//                             }
//                             dots[x as usize][y as usize] = false;
//                         }
//                     }

//                     curr_width /= 2;
//                 } else if side == "y" {
//                     let half_height: i32 = curr_height / 2;

//                     for x in 0..curr_width {
//                         for y in half_height..curr_height {
                            
//                             if dots[x as usize][y as usize] {
//                                 println!("Dot at position {},{}", x, y);
//                                 println!("Folding to put dot at position {},{}", x, (y-(curr_height-1)).abs());
//                                 dots[x as usize][(y-(curr_height-1)).abs() as usize] = dots[x as usize][y as usize];
//                             }
//                             dots[x as usize][y as usize] = false;
//                         }
//                     }

//                     curr_height /= 2;
//                 }

//                 // println!("AFTER FOLDING");
//                 break;
//             }
//         }

//         for i in 0..20 {
//             for j in 0..20 {
//                 if dots[j][i] {
//                     print!("#");
//                 } else {
//                     print!(".");
//                 }
//             }
//             println!();
//         }

//         let mut dot_count = 0;
//         for i in 0..curr_width {
//             for j in 0..curr_height {
//                 if dots[i as usize][j as usize] {
//                     dot_count += 1;
//                 }
//             }
//         }
        
//         println!("Number of dots = {}", dot_count);

//         // println!("Largest x = {} - largest y = {}", largest_x, largest_y);
//     }
// }
