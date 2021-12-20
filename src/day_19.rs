use crate::helpers::read_lines;

use crate::vec3::*;

#[derive(Clone, Debug)]
struct Scanner {
    coords: Vec<Vec3>,
    relative_distances: Vec<Vec<Vec3>>,
}

impl Scanner {
    pub fn new(coords: Vec<Vec3>) -> Self {
        let mut all_relative_distances = Vec::<Vec<Vec3>>::new();
        for i in 0..coords.len() {
            let mut relative_distances = Vec::<Vec3>::new();
            for j in 1..coords.len() {

                let relative = coords[i] - coords[j];
                relative_distances.push(relative.normalize());
            }

            all_relative_distances.push(relative_distances);
        }

        Scanner{ coords: coords, relative_distances: all_relative_distances }
    }
}

pub fn day_19_part_1() {
    let mut scanners = Vec::<Scanner>::new();

    if let Ok(file_lines) = read_lines("resources/day_19_test.txt") {
        
        let mut new_scanner = true;
        let mut curr_scanner = Scanner::new(vec![]);
        let mut curr_coords = Vec::<Vec3>::new();

        for file_line in file_lines {
            let line = file_line.unwrap();

            if new_scanner {
                curr_coords = Vec::<Vec3>::new();
                new_scanner = false;
                continue;
            }

            if line.len() == 0 {
                new_scanner = true;
                scanners.push(Scanner::new(curr_coords.clone()));
            } else {
                let mut split = line.split(",");
                let x = split.next().unwrap().parse::<i32>().unwrap() as f32;
                let y = split.next().unwrap().parse::<i32>().unwrap() as f32;
                let z = split.next().unwrap().parse::<i32>().unwrap() as f32;

                curr_coords.push(Vec3{ x, y, z });
            }
        }

        scanners.push(Scanner::new(curr_coords.clone()));
    }

    println!("Scanners length = {}", scanners.len());
    for i in 0..scanners.len() {
        println!("Scanner {} detected {} beacons", i, scanners[i].coords.len());
    }

    println!("{:?}", scanners[0]);

}
