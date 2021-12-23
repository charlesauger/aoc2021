use crate::helpers::read_lines;

use std::collections::{HashMap, HashSet};
use std::mem;

#[derive(Debug)]
struct Cuboid {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
}

fn intersection(cube_1: &Cuboid, cube_2: &Cuboid) {

}

pub fn day_22_part_2() {
    let mut final_cuboids = Vec::<Cuboid>::new();
    let mut working_cuboids = Vec::<Cuboid>::new();

    if let Ok(file_lines) = read_lines("resources/day_22.txt") {
        for file_line in file_lines {
            let line = file_line.unwrap();

            let mut on_split = line.split(" ");

            let on_or_off = on_split.next().unwrap();
            let turn_on = on_or_off == "on";

            let mut coords_split = on_split.next().unwrap().split(",");

            let mut x_coords_string_split = coords_split.next().unwrap().split("=");
            let mut y_coords_string_split = coords_split.next().unwrap().split("=");
            let mut z_coords_string_split = coords_split.next().unwrap().split("=");

            x_coords_string_split.next();
            let mut x_values_split = x_coords_string_split.next().unwrap().split("..");

            y_coords_string_split.next();
            let mut y_values_split = y_coords_string_split.next().unwrap().split("..");

            z_coords_string_split.next();
            let mut z_values_split = z_coords_string_split.next().unwrap().split("..");

            let mut x_min = x_values_split.next().unwrap().parse::<i64>().unwrap();
            let mut x_max = x_values_split.next().unwrap().parse::<i64>().unwrap();

            let mut y_min = y_values_split.next().unwrap().parse::<i64>().unwrap();
            let mut y_max = y_values_split.next().unwrap().parse::<i64>().unwrap();

            let mut z_min = z_values_split.next().unwrap().parse::<i64>().unwrap();
            let mut z_max = z_values_split.next().unwrap().parse::<i64>().unwrap();

            if x_min > x_max {
                mem::swap(&mut x_min, &mut x_max);
            }

            if y_min > y_max {
                mem::swap(&mut y_min, &mut y_max);
            }

            if z_min > z_max {
                mem::swap(&mut z_min, &mut z_max);
            }

            // println!("x_min={}, x_max={}", x_min, x_max);
            // println!("y_min={}, y_may={}", y_min, y_max);
            // println!("z_min={}, z_maz={}", z_min, z_max);

            working_cuboids.push(Cuboid{x_min: x_min, x_max: x_max , y_min: y_min, y_max: y_max, z_min: z_max, z_max: z_max});
        }
    }

    println!("{:?}", working_cuboids);
}

// pub fn day_22_part_1() {
//     const MIN_COORD: i64 = -50;
//     const MAX_COORD: i64 = 50;

//     let mut on_cubes = HashSet::<(i64, i64, i64)>::new();

//     let mut step = 1;

//     if let Ok(file_lines) = read_lines("resources/day_22.txt") {
//         for file_line in file_lines {
//             let line = file_line.unwrap();

//             let mut on_split = line.split(" ");

//             let on_or_off = on_split.next().unwrap();
//             let turn_on = on_or_off == "on";

//             let mut coords_split = on_split.next().unwrap().split(",");

//             let mut x_coords_string_split = coords_split.next().unwrap().split("=");
//             let mut y_coords_string_split = coords_split.next().unwrap().split("=");
//             let mut z_coords_string_split = coords_split.next().unwrap().split("=");

//             x_coords_string_split.next();
//             let mut x_values_split = x_coords_string_split.next().unwrap().split("..");

//             y_coords_string_split.next();
//             let mut y_values_split = y_coords_string_split.next().unwrap().split("..");

//             z_coords_string_split.next();
//             let mut z_values_split = z_coords_string_split.next().unwrap().split("..");

//             let x_min = x_values_split.next().unwrap().parse::<i64>().unwrap();
//             let x_max = x_values_split.next().unwrap().parse::<i64>().unwrap();

//             let y_min = y_values_split.next().unwrap().parse::<i64>().unwrap();
//             let y_max = y_values_split.next().unwrap().parse::<i64>().unwrap();

//             let z_min = z_values_split.next().unwrap().parse::<i64>().unwrap();
//             let z_max = z_values_split.next().unwrap().parse::<i64>().unwrap();

//             // println!("x_min={}, x_max={}", x_min, x_max);
//             // println!("y_min={}, y_may={}", y_min, y_max);
//             // println!("z_min={}, z_maz={}", z_min, z_max);

//             for x in x_min..=x_max {
//                 // if x < MIN_COORD || x > MAX_COORD {
//                 //     continue;
//                 // }

//                 for y in y_min..=y_max {
//                     // if y < MIN_COORD || y > MAX_COORD {
//                     //     continue;
//                     // }

//                     for z in z_min..=z_max {
//                         // if z < MIN_COORD || z > MAX_COORD {
//                         //     continue;
//                         // }

//                         if turn_on {
//                             on_cubes.insert((x, y, z));
//                         } else {
//                             on_cubes.remove(&(x, y, z));
//                         }
//                     }
//                 }
//             }

//             println!("On cubes after step {} = {}", step, on_cubes.len());
//             step += 1;
//         }
//     }

//     println!("Final amount of on cubes = {}", on_cubes.len());
// }
