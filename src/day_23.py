import sys


class Amp:
    def __init__(self, position, depth, goal, cost):
        self.position = position
        self.depth = depth
        self.goal = goal
        self.cost = cost
        self.moves = []

    def move(self, position, depth, move_cost, undo):
        self.position = position
        self.depth = depth
        if not undo:
            self.moves.append((position, depth, move_cost))
        else:
            self.moves.pop()

    def move_cost(self):
        move_cost = 0
        for move in self.moves:
            move_cost += move[2]
        return move_cost

    def character(self):
        if self.cost == 1:
            return "A"
        elif self.cost == 10:
            return "B"
        elif self.cost == 100:
            return "C"
        elif self.cost == 1000:
            return "D"


def calculate_total_move_cost(amps):
    total_move_cost = 0
    for key, amp in amps.items():
        for move in amp.moves:
            total_move_cost += move[2]
    return total_move_cost


def encode_state(amps):
    hallway_position_0 = "."
    hallway_position_1 = "."
    hallway_position_2_1 = "."
    hallway_position_2_2 = "."
    hallway_position_2_3 = "."
    hallway_position_2_4 = "."
    hallway_position_3 = "."
    hallway_position_4_1 = "."
    hallway_position_4_2 = "."
    hallway_position_4_3 = "."
    hallway_position_4_4 = "."
    hallway_position_5 = "."
    hallway_position_6_1 = "."
    hallway_position_6_2 = "."
    hallway_position_6_3 = "."
    hallway_position_6_4 = "."
    hallway_position_7 = "."
    hallway_position_8_1 = "."
    hallway_position_8_2 = "."
    hallway_position_8_3 = "."
    hallway_position_8_4 = "."
    hallway_position_9 = "."
    hallway_position_10 = "."

    for key, amp in amps.items():
        character = amp.character()
        if amp.position == 0:
            hallway_position_0 = character
        elif amp.position == 1:
            hallway_position_1 = character
        elif amp.position == 2:
            if amp.depth == 1:
                hallway_position_2_1 = character
            elif amp.depth == 2:
                hallway_position_2_2 = character
            if amp.depth == 3:
                hallway_position_2_3 = character
            elif amp.depth == 4:
                hallway_position_2_4 = character
        elif amp.position == 3:
            hallway_position_3 = character
        elif amp.position == 4:
            if amp.depth == 1:
                hallway_position_4_1 = character
            elif amp.depth == 2:
                hallway_position_4_2 = character
            if amp.depth == 3:
                hallway_position_4_3 = character
            elif amp.depth == 4:
                hallway_position_4_4 = character
        elif amp.position == 5:
            hallway_position_5 = character
        elif amp.position == 6:
            if amp.depth == 1:
                hallway_position_6_1 = character
            elif amp.depth == 2:
                hallway_position_6_2 = character
            if amp.depth == 3:
                hallway_position_6_3 = character
            elif amp.depth == 4:
                hallway_position_6_4 = character
        elif amp.position == 7:
            hallway_position_7 = character
        elif amp.position == 8:
            if amp.depth == 1:
                hallway_position_8_1 = character
            elif amp.depth == 2:
                hallway_position_8_2 = character
            if amp.depth == 3:
                hallway_position_8_3 = character
            elif amp.depth == 4:
                hallway_position_8_4 = character
        elif amp.position == 9:
            hallway_position_9 = character
        elif amp.position == 10:
            hallway_position_10 = character

    encoding = hallway_position_0 + hallway_position_1 + hallway_position_2_1 + hallway_position_2_2 + hallway_position_2_3 + hallway_position_2_4 + hallway_position_3 + hallway_position_4_1 + hallway_position_4_2 + hallway_position_4_3 + hallway_position_4_4  + hallway_position_5 + hallway_position_6_1 + hallway_position_6_2 + hallway_position_6_3 + hallway_position_6_4  + hallway_position_7 + hallway_position_8_1 + hallway_position_8_2 + hallway_position_8_3 + hallway_position_8_4 + hallway_position_9 + hallway_position_10
    encoding += str(calculate_total_move_cost(amps))
    return encoding


def print_amps(amps):
    print("--------")
    for key, amp in amps.items():
        print("Amp key {}:".format(key))
        print("Position: {} - Depth: {} - Goal: {} - Moves: {}".format(amp.position, amp.depth, amp.goal, amp.moves))


def amp_stuck_inside_room(key, amps):
    amp = amps[key]

    room_positions = [2, 4, 6, 8]

    if amp.position in room_positions:
        if amp.depth > 1:
            for other_key, other_amp in amps.items():
                if key != other_key:
                    # If the other amp is at the same position at a lower depth it is blocking the amp from moving
                    if other_amp.position == amp.position and other_amp.depth < amp.depth:
                        return True
    return False


def amp_can_move_to_goal(key, amps):
    amp = amps[key]

    if amp.position == amp.goal:
        # We are already in the goal so cannot move to it!
        return False

    if amp_stuck_inside_room(key, amps):
        # Stuck inside a room so no point in moving
        return False

    for other_key, other_amp in amps.items():
        if key != other_key:
            if amp.goal > amp.position:
                # Goal is to the right
                pass


def find_goal_depth_to_move_into(key, amps):
    amp = amps[key]

    if amp.position == amp.goal:
        # We are already in the goal so cannot move to it!
        return -1

    if amp_stuck_inside_room(key, amps):
        # Stuck inside a room so no point in moving
        return -1

    goal_empty = True
    lowest_goal_position_filled = 5 # Currently can only go to depth 4 so initialise to 1 more

    for other_key, other_amp in amps.items():
        if other_key != key:
            if other_amp.position == amp.goal:
                goal_empty = False

            if other_amp.position == amp.goal and other_amp.goal != amp.goal:
                # A different type of amp is in the goal so we can't move to it anyway
                return -1
            
            if other_amp.position == amp.goal:
                if other_amp.depth < lowest_goal_position_filled:
                    lowest_goal_position_filled = other_amp.depth
    
    assert lowest_goal_position_filled > 1
    return lowest_goal_position_filled - 1


def amp_in_good_goal_position(key, amps):
    amp = amps[key]

    if amp.position != amp.goal:
        return False

    for other_key, other_amp in amps.items():
        if other_key != key:
            if other_amp.position == amp.position and other_amp.goal != amp.goal and other_amp.depth > amp.depth:
                return False

    return True


def find_possible_goal_move(key, amps):
    possible_move = []

    amp = amps[key]

    if amp.position == amp.goal:
        # We are already in the goal so cannot move to it!
        return []

    if amp_stuck_inside_room(key, amps):
        # Stuck inside a room so no point in moving
        return []

    for other_key, other_amp in amps.items():
        if other_key != key:
            # Find if an amp is blocking the hallway path to the goal
            if amp.goal > amp.position:
                # Goal to the right
                if other_amp.position > amp.position and other_amp.position < amp.goal and other_amp.depth == 0:
                    return []
            elif amp.goal < amp.position:
                # Goal to the left
                if other_amp.position < amp.position and other_amp.position > amp.goal and other_amp.depth == 0:
                    return []

    goal_depth_to_move = find_goal_depth_to_move_into(key, amps)
    if goal_depth_to_move == -1:
        return []
    else:
        return [(amp.goal, goal_depth_to_move)]


def find_possible_hallway_moves(key, amps):
    # Assume that the amp can move out of the room it's in
    possible_moves = []

    amp = amps[key]

    hallway_positions = [0, 1, 3, 5, 7, 9, 10]

    if(not (amp.depth > 0 and amp.position not in hallway_positions)):
        print_amps(amps)
        assert False

    for hallway_position in hallway_positions:
        blocked = False
        for other_key, other_amp in amps.items():
            if other_key != key:
                if hallway_position > amp.position:
                    # position to the right
                    if other_amp.position > amp.position and other_amp.position < hallway_position and other_amp.depth == 0:
                        blocked = True
                        break
                elif hallway_position < amp.position:
                    # position to the left
                    if other_amp.position < amp.position and other_amp.position > hallway_position and other_amp.depth == 0:
                        blocked = True
                        break
                if other_amp.position == hallway_position:
                    blocked = True
                    break
        if not blocked:
            # We can move to hallway_position at depth 0
            possible_moves.append((hallway_position, 0))

    return possible_moves


def find_valid_moves(amps):
    valid_moves = []
    # loop through all the amps
    for key, amp in amps.items():
        possible_moves = []
        if amp.depth > 0:
            # We are in a room so the only move we can do is to go out

            if len(amp.moves) >= 2 and amp.position != amp.goal:
                # If we have moved twice we must have moved into our goal space
                assert False

            # If we are already in the goal and we don't need to move then continue
            if amp_in_good_goal_position(key, amps):
                continue

            if amp_stuck_inside_room(key, amps):
                # No point checking this amp if it's stuck inside a room
                continue

            possible_moves = find_possible_hallway_moves(key, amps)
            for move in possible_moves:
                valid_moves.append((key, (move[0], move[1])))
        else:
            # We are in the hallway so the only move we can do is go into our goal room
            if len(amp.moves) != 1:
                print_amps(amps)
                assert False
            
            possible_moves = find_possible_goal_move(key, amps)

        for move in possible_moves:
            valid_moves.append((key, (move[0], move[1])))
    return valid_moves


def calculate_move_cost(key, amps, move):
    total_cost = 0

    amp = amps[key]

    # Cost to move out of room
    total_cost += amp.depth * amp.cost

    # Cost to move to position
    total_cost += abs(amp.position - move[1][0]) * amp.cost

    # Cost to move into room
    total_cost += move[1][1] * amp.cost

    return total_cost


def solved(amps):
    for key, amp in amps.items():
        if amp.position != amp.goal:
            return False
    return True


def find_cheapest_cost(amps, depth):
    global cheapest_solved_cost
    global already_searched

    current_cost = calculate_total_move_cost(amps)
    if calculate_total_move_cost(amps) >= cheapest_solved_cost:
        # ABANDON THIS PATH
        return

    encoded_state = encode_state(amps)
    if encoded_state in already_searched:
        return

    already_searched.append(encoded_state)

    if solved(amps):
        if current_cost < cheapest_solved_cost:
            cheapest_solved_cost = current_cost
        print("Solved with cost: {}".format(current_cost))
        print_amps(amps)
        return

    # print("Searching at depth {} with current_cost = {}".format(depth, current_cost))

    valid_moves = find_valid_moves(amps)
    valid_moves = sorted(valid_moves, key=lambda m: calculate_move_cost(m[0], amps, m))

    i = 1

    for move in valid_moves:
        amp_to_move = amps[move[0]]
        original_position = amp_to_move.position
        original_depth = amp_to_move.depth

        move_cost = calculate_move_cost(move[0], amps, move)
        # Make the move and find the cheapest cost from this state
        amp_to_move.move(position=move[1][0], depth=move[1][1], move_cost=move_cost, undo=False)

        find_cheapest_cost(amps, depth + 1)

        # Undo the move for further testing
        amp_to_move.move(position=original_position, depth=original_depth, move_cost=None, undo=True)

        if depth == 1:
            print("Finished searching move branch {} of {} at depth 1".format(i, len(valid_moves)))
            i += 1

    # print("Cheapest cost found at depth {} = {}".format(depth, cheapest_cost_found))


amps = {1: Amp(2, 1, 2, 1),
        2: Amp(2, 2, 8, 1000),
        3: Amp(2, 3, 8, 1000),
        4: Amp(2, 4, 4, 10),

        5: Amp(4, 1, 8, 1000),
        6: Amp(4, 2, 6, 100),
        7: Amp(4, 3, 4, 10),
        8: Amp(4, 4, 6, 100),

        9: Amp(6, 1, 4, 10),
        10: Amp(6, 2, 4, 10),
        11: Amp(6, 3, 2, 1),
        12: Amp(6, 4, 2, 1),


        13: Amp(8, 1, 8, 1000),
        14: Amp(8, 2, 2, 1),
        15: Amp(8, 3, 6, 100),
        16: Amp(8, 4, 6, 100)}

already_searched = []
cheapest_solved_cost = sys.maxsize
depth = 1

find_cheapest_cost(amps, depth)
