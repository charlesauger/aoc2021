use crate::helpers::read_lines;

use std::collections::HashMap;

pub fn day_20_part_1() {
    let mut iea = vec![false; 512];

    let mut input_pixels = HashMap::<(i64, i64), bool>::new();

    if let Ok(file_lines) = read_lines("resources/day_20.txt") {

        let mut x = 0;
        let mut y = 0;

        let mut first = true;
        for file_line in file_lines {
            let line = file_line.unwrap();

            if line.len() == 0 {
                continue;
            }

            if first {
                let mut i = 0;
                for c in line.chars() {
                    match c {
                        '.' => iea[i] = false,
                        '#' => iea[i] = true,
                        _ => ()
                    }
                    i += 1;
                }

                first = false;

                continue;
            }

            for c in line.chars() {
                if c == '.' {
                    input_pixels.insert((x, y), false);
                } else if c == '#' {
                    input_pixels.insert((x, y), true);
                }

                x+= 1;
            }

            x = 0;
            y += 1;
        }
    }

    // println!("{:?}", iea);

    // println!("{:?}", input_pixels);

    let mut output_pixels = calculate_output_pixels(&input_pixels, &iea);

    input_pixels = output_pixels.clone();
    output_pixels = calculate_output_pixels(&input_pixels, &iea);
    
    draw_image(&output_pixels);

    let mut count = 0;
    for px in output_pixels {
        if px.1 {
            count += 1;
        }
    }

    println!("Count = {}", count);

}

fn calculate_output_pixels(input_pixels: &HashMap<(i64, i64), bool>, iea: &Vec<bool>) -> HashMap<(i64, i64), bool> {
    let mut output_pixels = HashMap::<(i64, i64), bool>::new();

    let boundaries = find_pixel_boundaries(&input_pixels);
    let (leftmost, rightmost, topmost, bottommost) = (boundaries.0, boundaries.1, boundaries.2, boundaries.3);

    // Calculate pixel at position based on input pixels
    for y in topmost..bottommost {
        for x in leftmost..rightmost {
            let pixels_to_check = [
                (x-1, y-1), (x, y-1), (x+1, y-1),
                (x-1, y), (x, y), (x+1, y),
                (x-1, y+1), (x, y+1), (x+1, y+1),
            ];

            let mut binary_string = String::new();
            for px in pixels_to_check {
                if !input_pixels.contains_key(&(px.0, px.1)) {
                    binary_string.push('0');
                } else {
                    let lit_pixel = input_pixels.get(&(px.0, px.1)).unwrap();
                    if *lit_pixel {
                        binary_string.push('1');
                    } else {
                        binary_string.push('0');
                    }
                }

            }

            let binary_index = i64::from_str_radix(binary_string.as_str(), 2).unwrap();
            // println!("Position ({},{}) - Binary index = {} - insert {}", x, y ,binary_index, iea[binary_index as usize]);

            output_pixels.insert((x, y), iea[binary_index as usize]);
        }
    }

    output_pixels
}

fn find_pixel_boundaries(pixels: &HashMap<(i64, i64), bool>) -> (i64, i64, i64, i64) {
    let mut leftmost = i64::MAX;
    let mut rightmost = i64::MIN;
    let mut topmost = i64::MAX;
    let mut bottommost = i64::MIN;

    for coords in pixels.keys() {
        if coords.0 < leftmost {
            leftmost = coords.0;
        }

        if coords.0 > rightmost {
            rightmost = coords.0;
        }

        if coords.1 < topmost {
            topmost = coords.1;
        }

        if coords.1 > bottommost {
            bottommost = coords.1;
        }
    }

    let modifier = 1;

    (leftmost-modifier, rightmost+modifier, topmost-modifier, bottommost+modifier)
}

fn draw_image(pixels: &HashMap<(i64, i64), bool>) {
    let boundaries = find_pixel_boundaries(&pixels);
    let (leftmost, rightmost, topmost, bottommost) = (boundaries.0, boundaries.1, boundaries.2, boundaries.3);

    for y in topmost..bottommost {
        for x in leftmost..rightmost {
            if !pixels.contains_key(&(x, y)) {
                print!(".");
            } else {
                let lit_pixel = pixels.get(&(x, y)).unwrap();
                if *lit_pixel {
                    print!("#");
                } else {
                    print!(".");
                }
            }

        }
        println!();
    }
}
