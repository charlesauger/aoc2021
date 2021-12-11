use crate::helpers::read_lines;

pub fn day_11_part_1() {
    if let Ok(file_lines) = read_lines("resources/day_11.txt") {

        let mut energy: [[usize; 10]; 10] = [[0; 10]; 10];
        let mut flashed: [[bool; 10]; 10] = [[false; 10]; 10];

        let mut y = 0;
        for file_line in file_lines {
            let line = file_line.unwrap();

            let mut x = 0;
            for c in line.chars() {
                energy[y][x] = c.to_digit(10).unwrap() as usize;
                
                x += 1;
            }

            y += 1;
        }

        for x in 0..10 {
            for y in 0..10 {
                print!("{}", energy[x][y]);
            }
            println!();
        }
        println!("-------------");
        println!("-------------");
        println!("-------------");

        let mut flash_count = 0;

        // Perform 100 steps
        for _i in 0..100 {
            for x in 0..10 {
                for y in 0..10 {
                    energy[x][y] += 1;
                }
            }

            for x in 0..10 {
                for y in 0..10 {
                    if energy[x][y] > 9 && !flashed[x][y] {
                        flash_position(x, y, &mut energy, &mut flashed);
                    }
                }
            }



            for x in 0..10 {
                for y in 0..10 {
                    if flashed[x][y] {
                        flash_count += 1;

                        energy[x][y] = 0;
                        flashed[x][y] = false;
                    }
                }
            }

            for x in 0..10 {
                for y in 0..10 {
                    print!("{}", energy[x][y]);
                }
                println!();
            }

            println!("-------------");
            println!("-------------");
            println!("-------------");
        }

        println!("Flash count: {}", flash_count);

    }

}

pub fn day_11_part_2() {
    if let Ok(file_lines) = read_lines("resources/day_11.txt") {

        let mut energy: [[usize; 10]; 10] = [[0; 10]; 10];
        let mut flashed: [[bool; 10]; 10] = [[false; 10]; 10];

        let mut y = 0;
        for file_line in file_lines {
            let line = file_line.unwrap();

            let mut x = 0;
            for c in line.chars() {
                energy[y][x] = c.to_digit(10).unwrap() as usize;
                
                x += 1;
            }

            y += 1;
        }

        for x in 0..10 {
            for y in 0..10 {
                print!("{}", energy[x][y]);
            }
            println!();
        }
        println!("-------------");
        println!("-------------");
        println!("-------------");

        let mut flash_count = 0;

        // Perform 100 steps
        for i in 0..1000 {
            for x in 0..10 {
                for y in 0..10 {
                    energy[x][y] += 1;
                }
            }

            for x in 0..10 {
                for y in 0..10 {
                    if energy[x][y] > 9 && !flashed[x][y] {
                        flash_position(x, y, &mut energy, &mut flashed);
                    }
                }
            }

            let mut all_flashed = true;
            for x in 0..10 {
                for y in 0..10 {
                    all_flashed &= flashed[x][y];
                }
            }
            if all_flashed {
                println!("All flashed on step: {}", i+1);
                break;
            }

            for x in 0..10 {
                for y in 0..10 {
                    if flashed[x][y] {
                        flash_count += 1;

                        energy[x][y] = 0;
                        flashed[x][y] = false;
                    }
                }
            }

            for x in 0..10 {
                for y in 0..10 {
                    print!("{}", energy[x][y]);
                }
                println!();
            }

            println!("-------------");
            println!("-------------");
            println!("-------------");
        }

        println!("Flash count: {}", flash_count);

    }

}

fn flash_position(x: usize, y: usize, energy: & mut[[usize; 10]; 10], flashed: & mut[[bool; 10]; 10]) {
    // println!("Flashing position {},{}", x, y);
    flashed[x][y] = true;

    let adjacent_positions = find_valid_adjacent_positions(x, y, 10, 10);

    for pos in adjacent_positions {
        energy[pos.0][pos.1] += 1;

        if !flashed[pos.0][pos.1] && energy[pos.0][pos.1] > 9 {
            // println!("{},{} is not flashed yet and has energy {} so we are flashing it", pos.0, pos.1, energy[pos.0][pos.1]);
            flash_position(pos.0, pos.1, energy, flashed);
        }
    }
}

fn find_valid_adjacent_positions(i: usize, j: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut valid_positions_int = Vec::<(i32, i32)>::new();

    let x = i as i32;
    let y = j as i32;

    valid_positions_int.push((x-1, y-1));
    valid_positions_int.push((x, y-1));
    valid_positions_int.push((x+1, y-1));

    valid_positions_int.push((x-1, y));
    valid_positions_int.push((x+1, y));

    valid_positions_int.push((x-1, y+1));
    valid_positions_int.push((x, y+1));
    valid_positions_int.push((x+1, y+1));

    let valid_positions_int_drained: Vec<(i32, i32)> = valid_positions_int.into_iter().filter(|pos| pos.0 >= 0 && pos.0 < w as i32 && pos.1 >= 0 && pos.1 < h as i32).collect();

    let mut valid_positions = Vec::<(usize, usize)>::new();
    for pos in valid_positions_int_drained {
        valid_positions.push((pos.0 as usize, pos.1 as usize));
    }

    valid_positions
}
