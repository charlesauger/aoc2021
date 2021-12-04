use crate::helpers::read_lines;

#[derive(Debug)]
struct BingoBoard {
    numbers: [[i32; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl BingoBoard {
    fn update(&mut self, number: i32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.numbers[i][j] == number {
                    self.marked[i][j] = true;
                }
            }
        }
    }

    // This is super inefficient, surely a better way... :P
    fn won(&mut self) -> bool {
        for i in 0..5 {
            let mut won = true;
            for j in 0..5 {
                won &= self.marked[i][j];
            }
            
            if won {
                return won;
            }
        }

        for j in 0..5 {
            let mut won = true;
            for i in 0..5 {
                won &= self.marked[i][j];
            }

            if won {
                return won;
            }
        }
        
        false
    }

    fn calculate_value(&self, last_number: i32) -> i32 {
        let mut value = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marked[i][j] {
                    value += self.numbers[i][j];
                }
            }
        }
        
        value *= last_number;
        value
    }
}

fn create_bingo_board(input: &Vec<String>) -> BingoBoard {
    let mut bingo_board = BingoBoard {
        numbers: [[0; 5]; 5],
        marked: [[false; 5]; 5],
    };

    let mut i = 0;
    
    for line in input {
        let mut j = 0;
        for num_string in line.split_whitespace() {
            let num: i32 = num_string.parse().unwrap();

            bingo_board.numbers[i][j] = num;

            j += 1;
        }

        // println!("{}", line);

        i += 1;
    }

    bingo_board
}



pub fn day_4_part_1() {

    if let Ok(mut lines) = read_lines("resources/day_4.txt") {

        // Read called numbers
        let called_numbers_line = lines.next().unwrap().unwrap();
        let called_numbers_list = called_numbers_line.split(',');
        let called_numbers: Vec<i32> = called_numbers_list.map(|value| value.parse::<i32>().unwrap()).collect();

        // Construct bingo boards
        let mut bingo_boards = Vec::<BingoBoard>::new();

        let mut lines_vec = Vec::<String>::new();

        for line_result in lines {
            if let Ok(line) = line_result {
                lines_vec.push(line);
            }
        }

        let mut current_board_strings = Vec::<String>::new();
        for line in lines_vec {
            if line.len() == 0 {
                continue;
            }

            current_board_strings.push(line.clone());

            if current_board_strings.len() == 5 {
                let board = create_bingo_board(&current_board_strings);
                bingo_boards.push(board);

                current_board_strings.clear();
            }
        }

        println!("Number of bingo boards = {}", bingo_boards.len());
        println!("Called numbers = {:?}", called_numbers);

        for i in called_numbers {
            for board in &mut bingo_boards {
                board.update(i);

                if board.won() {
                    println!("{} was just called, BINGO!", i);
                    println!("The value of the winning board is {}", board.calculate_value(i));
                    return;
                }
            }
        }
    }
}
