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
