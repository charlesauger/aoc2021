use crate::helpers::read_lines;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Probe {
    x: i64,
    y: i64,
    x_vel: i64,
    y_vel: i64,
}

impl Probe {
    fn Step(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;

        if self.x_vel > 0 {
            self.x_vel -= 1;
        } else if self.x_vel < 0 {
            self.x_vel += 1;
        }

        self.y_vel -= 1;

        // println!("After step probe in position {},{}", self.x, self.y);
    }

    fn ResetProbe(&mut self, x_vel: i64, y_vel: i64) {
        self.x = 0;
        self.y = 0;
        self.x_vel = x_vel;
        self.y_vel = y_vel;
    }
}

fn CreateProbe(x_vel: i64, y_vel: i64) -> Probe {
    Probe { x: 0, y: 0, x_vel: x_vel, y_vel: y_vel }
}

pub fn day_17_part_1() {
    let mut target_x_low = 0;
    let mut target_x_high = 0;

    let mut target_y_low = 0;
    let mut target_y_high = 0;

    if let Ok(file_lines) = read_lines("resources/day_17.txt") {
        
        for file_line in file_lines {
            let line = file_line.unwrap();

            let mut first_split = line.split(":");
            first_split.next();

            let mut second_split = first_split.next().unwrap().split(",");

            let x_values = second_split.next().unwrap();
            let y_values = second_split.next().unwrap();

            let mut x_values_split = x_values[3..].split("..");
            let x_1 = x_values_split.next().unwrap();
            let x_2 = x_values_split.next().unwrap();

            let mut y_values_split = y_values[3..].split("..");
            let y_1 = y_values_split.next().unwrap();
            let y_2 = y_values_split.next().unwrap();

            println!("x={}..{} y={}..{}", x_1, x_2, y_1, y_2);

            let x_1_int = x_1.parse::<i64>().unwrap();
            let x_2_int = x_2.parse::<i64>().unwrap();
            let y_1_int = y_1.parse::<i64>().unwrap();
            let y_2_int = y_2.parse::<i64>().unwrap();

            if x_1_int < x_2_int {
                target_x_low = x_1_int;
                target_x_high = x_2_int;
            } else {
                target_x_low = x_2_int;
                target_x_high = x_1_int;
            }

            if y_1_int < y_2_int {
                target_y_low = y_1_int;
                target_y_high = y_2_int;
            } else {
                target_y_low = y_2_int;
                target_y_high = y_1_int;
            }
        }
    }

    // let mut good_x_vels = HashMap::<i64, Vec::<i64>>::new();
    let mut good_x_vels = HashSet::<i64>::new();

    for x_vel in 0..target_x_high+1 {
        let mut probe = CreateProbe(x_vel, 0);

        let mut previous_x = probe.x;
        while true {
            probe.Step();
            if probe.x <= previous_x {
                // println!("x_vel of {} failed to reach x_target", x_vel);
                break;
            }

            if probe.x >= target_x_low && probe.x <= target_x_high {
                // println!("x_vel of {} within target at x position {}", x_vel, probe.x);

                // HASHMAP
                // if !good_x_vels.contains_key(&x_vel) {
                //     good_x_vels.insert(x_vel, vec![probe.x]);
                // } else {
                //     let mut curr_list = good_x_vels.get(&x_vel).unwrap().clone();
                //     curr_list.push(probe.x);
                //     good_x_vels.insert(x_vel, curr_list);
                // }

                // HASHSET
                good_x_vels.insert(x_vel);
                break;
            }

            if probe.x > target_x_high {
                // println!("Probe with x_vel {} has flown past target x, final position = {}", x_vel, probe.x);
                break;
            }

            previous_x = probe.x;
        }
    }

    // let mut good_y_vels = HashMap::<i64, Vec::<i64>>::new();
    let mut good_y_vels = HashSet::<i64>::new();

    // Maybe we can do something smart for the max here...
    for y_vel in target_y_low..1000 {
        let mut probe = CreateProbe(0, y_vel);

        let mut previous_y = probe.y;
        while true {
            probe.Step();

            if probe.y >= target_y_low && probe.y <= target_y_high {
                // println!("y_vel of {} within target at y position {}", y_vel, probe.y);

                // HASHMAP
                // if !good_y_vels.contains_key(&y_vel) {
                //     good_y_vels.insert(y_vel, vec![probe.y]);
                // } else {
                //     let mut curr_list = good_y_vels.get(&y_vel).unwrap().clone();
                //     curr_list.push(probe.y);
                //     good_y_vels.insert(y_vel, curr_list);
                // }

                // HASHSET
                good_y_vels.insert(y_vel);
                break;
            }

            if probe.y < target_y_low {
                // println!("Probe with y_vel {} has flown past target y, final position = {}", y_vel, probe.y);
                break;
            }

            previous_y = probe.y;
        }
    }

    println!("good x vels = {:?}", good_x_vels);
    println!("good y vels = {:?}", good_y_vels);

    let mut valid_initial_vels = HashSet::<(i64, i64)>::new();

    for x_vel in &good_x_vels {
        for y_vel in &good_y_vels {
            
            // HASHMAP
            // let curr_x_vel = x_vel.0.clone();
            // let curr_y_vel = y_vel.0.clone();

            // HASHSET
            let curr_x_vel = x_vel.clone();
            let curr_y_vel = y_vel.clone();

            let mut probe = CreateProbe(curr_x_vel, curr_y_vel);

            while true {
                probe.Step();

                if probe.x > target_x_high || probe.y < target_y_low {
                    // println!("Real probe with starting x_vel {} and y_vel {} has left the target area", curr_x_vel, curr_y_vel);
                    break;
                }

                if probe.x >= target_x_low && probe.x <= target_x_high && probe.y >= target_y_low && probe.y <= target_y_high {
                    // println!("Real probe has entered the target area at position {},{} - had starting x_vel {} and y_vel {}", probe.x, probe.y, curr_x_vel, curr_y_vel);
                    valid_initial_vels.insert((curr_x_vel, curr_y_vel));
                }
            }
        }
    }

    println!("List of valid initial vels = {:?}", valid_initial_vels);
    println!("Valid initial velocities length = {}", valid_initial_vels.len());

    let mut highest_y_position = i64::MIN;
    for (x_vel, y_vel) in valid_initial_vels {
        let mut probe = CreateProbe(x_vel, y_vel);

        while true {
            probe.Step();

            if probe.x > target_x_high || probe.y < target_y_low {
                break;
            }

            if probe.y > highest_y_position {
                highest_y_position = probe.y;
            }
        }
    }

    println!("Highest y position = {}", highest_y_position);
}
