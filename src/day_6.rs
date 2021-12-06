use crate::helpers::read_lines;

#[derive(Debug)]
struct LanternFish {
    days_until_spawn: u8
}

fn print_lanternfish(mut lantern_fish: &Vec<LanternFish>) {
    for l in lantern_fish {
        print!("{},", l.days_until_spawn);
    }
}

pub fn day_6_part_1() {
    if let Ok(mut file_lines) = read_lines("resources/day_6.txt") {
        let file_line = file_lines.next().unwrap().unwrap();

        let mut lantern_fish = Vec::<LanternFish>::new();

        let split = file_line.split(",");
        for s in split {
            lantern_fish.push(LanternFish{ days_until_spawn: s.parse::<u8>().unwrap() })
        }

        // print!("Initial state: ");
        // print_lanternfish(&lantern_fish);
        // println!();

        for i in 1..=80 {
            let mut number_of_new_fish = 0;
            lantern_fish.iter_mut().for_each(|l| {
                if l.days_until_spawn == 0 {
                    number_of_new_fish += 1;
                    l.days_until_spawn = 6;
                } else {
                    l.days_until_spawn -= 1;
                }
            });
    
            for _i in 0..number_of_new_fish {
                lantern_fish.push(LanternFish{ days_until_spawn: 8 });
            }
    
            // if i > 1 {
            //     print!("After {} days: ", i)
            // } else {
            //     print!("After {} day: ", i);
            // }
            // print_lanternfish(&lantern_fish);
            // println!();
        }

        println!("Total number of Lanternfish after 80 days = {}", lantern_fish.len());
    }
}

pub fn day_6_part_2() {
    if let Ok(mut file_lines) = read_lines("resources/day_6.txt") {
        let file_line = file_lines.next().unwrap().unwrap();

        let mut lantern_fish: [u64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

        let split = file_line.split(",");
        for s in split {
            lantern_fish[s.parse::<usize>().unwrap()] += 1;
        }

        // print!("Initial state: ");
        // print_lanternfish(&lantern_fish);
        // println!();

        for i in 1..=256 {
            let number_of_new_fish = lantern_fish[0];

            for i in 0..8 {
                lantern_fish[i] = lantern_fish[i+1];
            }

            // All the fish which spawned new fish get added to the 6 days element
            lantern_fish[6] += number_of_new_fish;

            // Add all the spawned fish
            lantern_fish[8] = number_of_new_fish;
            
            // if i > 1 {
            //     print!("After {} days: ", i)
            // } else {
            //     print!("After {} day: ", i);
            // }
            // print_lanternfish(&lantern_fish);
            // println!();
        }

        println!("Total number of Lanternfish after 256 days = {}", lantern_fish.iter().sum::<u64>());
    }
}
