use crate::helpers::read_lines;

pub fn day_9_part_1() {
    if let Ok(file_lines) = read_lines("resources/day_9.txt") {

        // Constants for day_9_test.txt
        // const W: i32 = 10;
        // const H: i32 = 5;

        // Constants for day_9.txt
        const W: i32 = 100;
        const H: i32 = 100;

        let mut heights = [[0; W as usize]; H as usize];

        let mut h = 0;
        for file_line in file_lines {
            let line = file_line.unwrap();

            let mut w = 0;
            for c in line.chars() {
                heights[h][w] = c.to_digit(10).unwrap();

                w +=1;
            }

            h += 1;
        }

        // println!("{:?}", heights);

        let mut sum = 0;

        for i in 0..H {
            for j in 0..W {
                
                let mut positions_to_search = Vec::<(i32, i32)>::new();

                if (i - 1) >= 0 {
                    positions_to_search.push((i-1, j));
                }

                if (j -1) >= 0 {
                    positions_to_search.push((i, j-1));
                }

                if i < (H - 1) {
                    positions_to_search.push((i+1, j));
                }

                if j < (W - 1) {
                    positions_to_search.push((i, j+1));
                }

                // println!("{:?}", positions_to_search);

                let mut lowest = true;
                let current_height = heights[i as usize][j as usize];
                for pos in positions_to_search {
                    if current_height >= heights[pos.0 as usize][pos.1 as usize] {
                        lowest = false;
                        break;
                    }
                }

                if lowest {
                    sum += current_height + 1;
                }
            }
        }

        println!("The sum of the risk levels is {}", sum);

    }

}

fn find_valid_adjacent_positions(i: i32, j: i32, w: i32, h: i32) -> Vec<(i32, i32)> {
    let mut valid_positions = Vec::<(i32, i32)>::new();
    
    if (i - 1) >= 0 {
        valid_positions.push((i-1, j));
    }

    if (j -1) >= 0 {
        valid_positions.push((i, j-1));
    }

    if i < (h - 1) {
        valid_positions.push((i+1, j));
    }

    if j < (w - 1) {
        valid_positions.push((i, j+1));
    }

    valid_positions
}

pub fn day_9_part_2() {
    if let Ok(file_lines) = read_lines("resources/day_9.txt") {

        // Constants for day_9_test.txt
        // const W: i32 = 10;
        // const H: i32 = 5;

        // Constants for day_9.txt
        const W: i32 = 100;
        const H: i32 = 100;

        let mut heights: [[i32; W as usize]; H as usize] = [[0; W as usize]; H as usize];

        let mut h = 0;
        for file_line in file_lines {
            let line = file_line.unwrap();

            let mut w = 0;
            for c in line.chars() {
                heights[h][w] = c.to_digit(10).unwrap() as i32;

                w +=1;
            }

            h += 1;
        }

        // println!("{:?}", heights);

        let mut lowest_positions = Vec::<(i32, i32)>::new();

        for i in 0..H {
            for j in 0..W {
                
                let mut positions_to_search = Vec::<(i32, i32)>::new();

                if (i - 1) >= 0 {
                    positions_to_search.push((i-1, j));
                }

                if (j -1) >= 0 {
                    positions_to_search.push((i, j-1));
                }

                if i < (H - 1) {
                    positions_to_search.push((i+1, j));
                }

                if j < (W - 1) {
                    positions_to_search.push((i, j+1));
                }

                // println!("{:?}", positions_to_search);

                let mut lowest = true;
                let current_height = heights[i as usize][j as usize];
                for pos in positions_to_search {
                    if current_height >= heights[pos.0 as usize][pos.1 as usize] {
                        lowest = false;
                        break;
                    }
                }

                if lowest {
                    lowest_positions.push((i, j));
                }
            }
        }

        println!("Lowest positions: {:?}", lowest_positions);

        let mut basin_sizes = Vec::<usize>::new();

        for pos in lowest_positions {
            // Find the size of the basin

            let mut positions_to_search = Vec::<(i32, i32)>::new();
            let mut searched_positions = Vec::<(i32, i32)>::new();
            let mut basin_positions = Vec::<(i32, i32)>::new();
            searched_positions.push(pos);
            basin_positions.push(pos);

            // Find positions adjacent to lowest position
            let mut adjacent_positions = find_valid_adjacent_positions(pos.0, pos.1, W, H);
            for pos in adjacent_positions {
                if !searched_positions.contains(&pos) {
                    positions_to_search.push(pos.clone());
                }
            }

            // println!("Initial positions_to_search = {:?}", positions_to_search);

            while positions_to_search.len() > 0 {
                let search_pos = positions_to_search.pop().unwrap();
                
                searched_positions.push(search_pos.clone());

                if heights[search_pos.0 as usize][search_pos.1 as usize] != 9 {
                    // println!("Search position is currently {:?} and its height is not 9, adding to basin_positions", search_pos);
                    basin_positions.push(search_pos.clone());

                    adjacent_positions = find_valid_adjacent_positions(search_pos.0, search_pos.1, W, H);
                    for pos in adjacent_positions {
                        if !searched_positions.contains(&pos) && !positions_to_search.contains(&pos) {
                            // println!("Searched positions = {:?}", searched_positions);
                            // println!("Adding position {:?} to positions_to_search", pos);
                            positions_to_search.push(pos.clone());
                        }
                    }
                }
            }

            println!("Basin positions = {:?}", basin_positions);
            println!("-----------------");

            basin_sizes.push(basin_positions.len());
        }

        basin_sizes.sort_by(|a, b| b.cmp(a));

        let three_largest_mult = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];
        println!("Three largest basin sizes multiplied together = {}", three_largest_mult);
    }
}


