use crate::helpers::read_lines;

pub fn day_21_part_1() {
    let mut player_positions = [0, 0];
    if let Ok(file_lines) = read_lines("resources/day_21.txt") {
        let mut first = true;
        for file_line in file_lines {
            let line = file_line.unwrap();
            let mut split = line.split(":");
            split.next();

            if first {
                player_positions[0] = split.next().unwrap().trim().parse::<usize>().unwrap() - 1;
                first = false;
            } else {
                player_positions[1] = split.next().unwrap().trim().parse::<usize>().unwrap() - 1;
            }
        }
    }

    println!("{} {}", player_positions[0], player_positions[1]);

    let position_scores: [usize; 10] = [1,2,3,4,5,6,7,8,9,10];
    let mut die_values = Vec::<usize>::new();
    for i in 1..=100 {
        die_values.push(i);
    }
    let mut die_pointer = 0;

    let mut player_scores: [usize; 2] = [0, 0];
    let mut player_turn = 0;

    let mut num_dice_rolls = 0;

    const DICE_ROLLS_PER_TURN: usize = 3;

    loop {
        let mut amount_to_move = 0;
        for _i in 0..DICE_ROLLS_PER_TURN {
            let roll = die_values[die_pointer];
            amount_to_move += roll;
            die_pointer = (die_pointer + 1) % die_values.len();
        }
        num_dice_rolls += DICE_ROLLS_PER_TURN;

        // Move the player
        player_positions[player_turn] = (player_positions[player_turn] + amount_to_move) % 10;

        // Increase the player's score
        player_scores[player_turn] += position_scores[player_positions[player_turn]];

        // End the game if the player has reached 1000 points
        if player_scores[player_turn] >= 1000 {
            let winning_player_score = player_scores[player_turn];
            println!("Player {} wins with score {}!", player_turn + 1, winning_player_score);

            let losing_player_score = player_scores[(player_turn + 1) % 2];
            println!("Losing player {} had score {}", (player_turn + 1) % 2, losing_player_score);

            println!("Losing player score * number of dice rolls = {} * {} = {}", losing_player_score, num_dice_rolls, losing_player_score * num_dice_rolls);
            break;
        }

        player_turn = (player_turn + 1) % 2;
    }
}

// Possible rolls
//
// first roll  second roll  third roll
// 1 1 1 = 3
// 1 1 2 = 4
// 1 1 3 = 5
// 1 2 1 = 4
// 1 2 2 = 5
// 1 2 3 = 6
// 1 3 1 = 5
// 1 3 2 = 6
// 1 3 3 = 7
// 2 1 1 = 4
// 2 1 2 = 5
// 2 1 3 = 6
// 2 2 1 = 5
// 2 2 2 = 6
// 2 2 3 = 7
// 2 3 1 = 6
// 2 3 2 = 7
// 2 3 3 = 8
// 3 1 1 = 5
// 3 1 2 = 6
// 3 1 3 = 7
// 3 2 1 = 6
// 3 2 2 = 7
// 3 2 3 = 8
// 3 3 1 = 7
// 3 3 2 = 8
// 3 3 3 = 9
//
// 3 = 1x   4 = 3x     5 = 6x    6 =  7x   7 = 6x     8 = 3x      9 = 1x

fn add_pair(mut output: &mut (usize, usize), other: &(usize, usize)) {
    output.0 += other.0;
    output.1 += other.1;
}

fn multiply_pair(mut pair: &mut (usize, usize), multiplier: usize) {
    pair.0 *= multiplier;
    pair.1 *= multiplier;
}

fn calculate_winning_universes(mut player_1_score: usize, mut player_2_score: usize, mut player_1_pos: usize, mut player_2_pos: usize, player_turn: usize, total_dice_roll: usize) -> (usize, usize) {
    const POSITION_SCORES: [usize; 10] = [1,2,3,4,5,6,7,8,9,10];

    const NUMBER_OF_3_ROLLS: usize = 1;
    const NUMBER_OF_4_ROLLS: usize = 3;
    const NUMBER_OF_5_ROLLS: usize = 6;
    const NUMBER_OF_6_ROLLS: usize = 7;
    const NUMBER_OF_7_ROLLS: usize = 6;
    const NUMBER_OF_8_ROLLS: usize = 3;
    const NUMBER_OF_9_ROLLS: usize = 1;

    if player_turn == 0 {
        player_1_pos = (player_1_pos + total_dice_roll) % 10;
        player_1_score += POSITION_SCORES[player_1_pos];

        if player_1_score >= 21 {
            return (1, 0);
        } else {
        }
    } else {
        player_2_pos = (player_2_pos + total_dice_roll) % 10;
        player_2_score += POSITION_SCORES[player_2_pos];

        if player_2_score >= 21 {
            return (0, 1);
        }
    }

    
    let mut winning_universes_with_3_roll = calculate_winning_universes(player_1_score, player_2_score, player_1_pos, player_2_pos, (player_turn + 1) % 2, 3);
    let mut winning_universes_with_4_roll = calculate_winning_universes(player_1_score, player_2_score, player_1_pos, player_2_pos, (player_turn + 1) % 2, 4);
    let mut winning_universes_with_5_roll = calculate_winning_universes(player_1_score, player_2_score, player_1_pos, player_2_pos, (player_turn + 1) % 2, 5);
    let mut winning_universes_with_6_roll = calculate_winning_universes(player_1_score, player_2_score, player_1_pos, player_2_pos, (player_turn + 1) % 2, 6);
    let mut winning_universes_with_7_roll = calculate_winning_universes(player_1_score, player_2_score, player_1_pos, player_2_pos, (player_turn + 1) % 2, 7);
    let mut winning_universes_with_8_roll = calculate_winning_universes(player_1_score, player_2_score, player_1_pos, player_2_pos, (player_turn + 1) % 2, 8);
    let mut winning_universes_with_9_roll = calculate_winning_universes(player_1_score, player_2_score, player_1_pos, player_2_pos, (player_turn + 1) % 2, 9);

    multiply_pair(&mut winning_universes_with_3_roll, NUMBER_OF_3_ROLLS);
    multiply_pair(&mut winning_universes_with_4_roll, NUMBER_OF_4_ROLLS);
    multiply_pair(&mut winning_universes_with_5_roll, NUMBER_OF_5_ROLLS);
    multiply_pair(&mut winning_universes_with_6_roll, NUMBER_OF_6_ROLLS);
    multiply_pair(&mut winning_universes_with_7_roll, NUMBER_OF_7_ROLLS);
    multiply_pair(&mut winning_universes_with_8_roll, NUMBER_OF_8_ROLLS);
    multiply_pair(&mut winning_universes_with_9_roll, NUMBER_OF_9_ROLLS);

    let mut final_output = (0, 0);
    add_pair(&mut final_output, &winning_universes_with_3_roll);
    add_pair(&mut final_output, &winning_universes_with_4_roll);
    add_pair(&mut final_output, &winning_universes_with_5_roll);
    add_pair(&mut final_output, &winning_universes_with_6_roll);
    add_pair(&mut final_output, &winning_universes_with_7_roll);
    add_pair(&mut final_output, &winning_universes_with_8_roll);
    add_pair(&mut final_output, &winning_universes_with_9_roll);
    final_output
}

pub fn day_21_part_2() {

    let mut player_positions = [0, 0];
    if let Ok(file_lines) = read_lines("resources/day_21.txt") {
        let mut first = true;
        for file_line in file_lines {
            let line = file_line.unwrap();
            let mut split = line.split(":");
            split.next();

            if first {
                player_positions[0] = split.next().unwrap().trim().parse::<usize>().unwrap() - 1;
                first = false;
            } else {
                player_positions[1] = split.next().unwrap().trim().parse::<usize>().unwrap() - 1;
            }
        }
    }

    const NUMBER_OF_ROLLS: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

    let mut results = Vec::<(usize, usize)>::new();

    for roll in 3..=9 {
        let mut result = calculate_winning_universes(
            0,
            0,
            player_positions[0],
            player_positions[1],
            0,
            roll
        );

        multiply_pair(&mut result, NUMBER_OF_ROLLS[roll]);

        results.push(result);
    }

    println!("Results = {:?}", results);

    let mut first_player_wins: usize = 0;
    let mut second_player_wins: usize = 0;

    for result in results {
        first_player_wins += result.0;
        second_player_wins += result.1;
    }

    println!("First player wins in {} universes", first_player_wins);
    println!("Second player wins in {} universes", second_player_wins);
}
