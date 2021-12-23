#[derive(Copy, Clone, Debug)]
struct Amphipod {
    position: usize,
    depth: usize, // how deep in its current position is it?
    goal_position: usize,
    movement_cost: usize,
}

#[derive(Copy, Clone, Debug)]
struct Position {
    position: usize,
    depth: usize,
}

impl Amphipod {
    fn new(position: usize, depth: usize, goal_position: usize, movement_cost: usize) -> Self {
        Amphipod{ position, depth, goal_position, movement_cost }
    }

    fn find_valid_move_positions(&self, amphipods: &Vec<Amphipod>) -> Vec<Position> {
        let mut valid_positions = Vec::<Position>::new();

        // No need to ever move if we're in our goal at the highest depth
        if self.position == self.goal_position && self.depth == 2 {
            // Already at goal, no need to move
            return Vec::<Position>::new();
        }

        // No need to ever move if we're in our goal at the lowest depth
        // and another amphipod with the same goal is in the same room at the highest depth
        if self.position == self.goal_position && self.depth == 1 {
            for amphipod in amphipods {
                // Don't count yourself
                if amphipod.position == self.position && amphipod.depth == self.depth {
                    continue;
                }
    
                if amphipod.goal_position == self.goal_position && amphipod.position == self.goal_position && amphipod.depth == 2 {
                    // Already at goal, no need to move
                    return Vec::<Position>::new();
                }
            }
        }

        // We cannot move if we are blocked
        if self.depth == 2 {
            for amphipod in amphipods {
                // Don't count yourself
                if amphipod.position == self.position && amphipod.depth == self.depth {
                    continue;
                }
    
                if amphipod.position == self.position && amphipod.depth == 1 {
                    return Vec::<Position>::new();
                }
            }
        }

        let hallway_positions = vec![0, 1, 3, 5, 7, 9, 10];
        let in_hallway = hallway_positions.contains(&self.position);

        if in_hallway {
            let mut goal_open = true;
            let mut goal_empty = true;
            let mut goal_blocked = false;

            // We can only move to a room position.
            // Only move into a room if it is empty or there is another Amphipod in the room at depth 2
            for amphipod in amphipods {
                // Don't count yourself
                if amphipod.position == self.position && amphipod.depth == self.depth {
                    continue;
                }

                if amphipod.position == self.goal_position {
                    if amphipod.depth == 1 {
                        goal_open = false;
                    }
                    goal_empty = false;
                }

                // Goal positions to the left
                if self.goal_position < self.position {
                    // If we are blocked from moving to the goal position currently
                    if amphipod.position > self.goal_position && amphipod.depth == 0 {
                        goal_blocked = true;
                    }
                }
                // Goal positions to the right 
                else if self.goal_position > self.position {
                    // If we are blocked from moving to the goal position currently
                    if amphipod.position < self.goal_position && amphipod.depth == 0 {
                        goal_blocked = true;
                    }
                }
            }

            if !goal_blocked && goal_open {
                if goal_empty {
                    // we can try move into the depth 2 position
                    valid_positions.push(Position{position: self.goal_position, depth:2});
                } else {
                    // move into the depth 1 position
                    valid_positions.push(Position{position: self.goal_position, depth:1});
                }
            }
        } else {
            // We can move to any empty non-blocked hallway position
            // If we are not blocked by another Amphipod in the same position at a lower depth
            // Or by another amphipod in the hallway blocking us
            
            let mut stuck_in_room = false;
            for amphipod in amphipods {
                if amphipod.position == self.position && amphipod.depth < self.depth {
                    stuck_in_room = true;
                }
            }

            if !stuck_in_room {
                for pos in hallway_positions {
                    let mut blocked = false;

                    for amphipod in amphipods {
                        // Hallway positions to the left
                        if pos < self.position {
                            if amphipod.position >= pos && amphipod.position == 0 {
                                blocked = true;
                                break;
                            }
                        }
                        // Hallway positions to the right
                        else if pos > self.position {
                            if amphipod.position <= pos && amphipod.position == 0 {
                                blocked = true;
                                break;
                            }
                        }
                    }

                    if !blocked {
                        valid_positions.push(Position{position: pos, depth: 0});
                    }
                }
            }
        }

        valid_positions
    }

    fn distance_to_position(&self, position: &Position) -> usize {
        let mut distance = 0;
        if self.position < position.position {
            distance += position.position - self.position;
        } else {
            distance += self.position - position.position;
        }

        if self.depth < position.depth {
            distance += position.depth - self.depth;
        } else {
            distance += self.depth - position.depth;
        }

        distance * self.movement_cost
    }

    fn change_position(&mut self, position: Position) {
        self.position = position.position;
        self.depth = position.depth;
    }
}

fn solved(amphipods: &Vec<Amphipod>) -> bool {
    for amphipod in amphipods {
        if amphipod.position != amphipod.goal_position {
            return false;
        }
    }

    true
}

// Returns the lowest cost of the recursive tree from making this move
fn find_lowest_cost_solution(amphipods: &Vec<Amphipod>) -> usize {
    // If the puzzle is solved then it costs 0 from this point
    if solved(&amphipods) {
        return 0;
    }

    let mut lowest_cost = usize::MAX;

    let cloned_amphipods = amphipods.clone();

    for amphipod in amphipods {
        let valid_moves = amphipod.find_valid_move_positions(&cloned_amphipods);

        let current_position = Position{position: amphipod.position, depth: amphipod.depth};

        for mov in valid_moves {
            let mut move_cost = amphipod.distance_to_position(&mov);

            amphipod.change_position(mov);

            let mut amphi_clone = amphipods.clone();

            amphipod.change_position(current_position.clone());

            move_cost += find_lowest_cost_solution(&mut amphi_clone);

            if move_cost < lowest_cost {
                lowest_cost = move_cost;
            }
        }
    }

    lowest_cost
}
 

pub fn day_23_part_1() {
    let mut amphipods = Vec::<Amphipod>::new();

    amphipods.push(Amphipod::new(2, 1, 4, 10));
    amphipods.push(Amphipod::new(2, 2,  2, 1));
    amphipods.push(Amphipod::new(4, 1, 6, 100));
    amphipods.push(Amphipod::new(4, 2, 8, 1000));
    amphipods.push(Amphipod::new(6, 1, 4, 10));
    amphipods.push(Amphipod::new(6, 2, 6, 100));
    amphipods.push(Amphipod::new(8, 1, 8, 1000));
    amphipods.push(Amphipod::new(8, 2, 2, 1));

    for amphipod in &amphipods {
        let valid_moves = amphipod.find_valid_move_positions(&amphipods.clone());
        
        println!("{:?}", valid_moves);
    }

    println!("{}", find_lowest_cost_solution(&mut amphipods));

}
