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
