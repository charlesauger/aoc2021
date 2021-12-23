import sys

class Amp:
    def __init__(self, position, depth, goal, cost):
        self.position = position
        self.depth = depth
        self.goal = goal
        self.cost = cost
        self.move_count = 0
        self.moves = []

    def distance_to_position(self, position, depth):
        distance = 0
        if self.position < position:
            distance += position - self.position
        else:
            distance += self.position - position
        
        if self.depth < depth:
            distance += depth - self.depth
        else:
            distance += self.depth - depth
        
        return distance * self.cost

    def change_position(self, position, depth):
        self.position = position
        self.depth = depth;


def find_valid_move_positions(amps):
    valid_positions = []

    for key, amp in amps.items():
        # find all positions this amp can move to

        if amp.move_count == 2:
            print("Amp {} position = {} = Amp goal = {}".format(key, amp.position, amp.goal))
            for other_key, other_amp in amps.items():
                print("Amp {} position = {} goal = {} move_count = {} moves = {}".format(other_key, other_amp.position, other_amp.goal, other_amp.move_count, other_amp.moves))
            assert amp.position == amp.goal
            continue

        # don't need to move this amp if it's already at its deepest goal position
        if amp.position == amp.goal and amp.depth == 2:
            continue

        # don't need to move this amp if it's at it's goal depth 1 and another of the same
        # type is at its goal depth 2
        if amp.position == amp.goal and amp.depth == 1:
            for other_key, other_amp in amps.items():
                if other_amp is not key:
                    if other_amp.goal == amp.goal and other_amp.position == other_amp.goal and other_amp.depth == 2:
                        continue
        
        # don't move if blocked inside
        if amp.depth == 2:
            for other_key, other_amp in amps.items():
                if other_key is not key:
                    if other_amp.position == amp.position and other_amp.depth == 1:
                        continue

        hallway_positions = [0, 1, 3, 5, 7, 9, 10]
        in_hallway = amp.position in hallway_positions

        if in_hallway:
            goal_open = True
            goal_empty = True
            goal_blocked = False

            # In the hallway so only allowed to move to our goal position
            for other_key, other_amp in amps.items():
                if other_key is not key:
                    if other_amp.position == amp.goal:
                        if other_amp.depth == 1:
                            goal_open = False
                        goal_empty = False
                    
                    if amp.goal < amp.position:
                        # Goal position to left
                        # If we are blocked from moving to the goal position currently
                        if other_amp.position > amp.goal and other_amp.depth == 0:
                            goal_blocked = True
                    elif amp.goal > amp.position:
                        # Goal position to right
                        # If we are blocked from moving to the goal position currently
                        if other_amp.position < amp.goal and other_amp.depth == 0:
                            goal_blocked = True

            if not goal_blocked and goal_open:
                if goal_empty:
                    valid_positions.append((key, (amp.goal, 2)))
                else:
                    valid_positions.append((key, (amp.goal, 1)))
        else:
            # In a room
            # We can move to any empty non-blocked hallway position
            # If we are not blocked by another amp in the same position at a lower depth
            # or by another amp in the hallway blocking us

            # Can't move if stuck in a room
            for other_key, other_amp in amps.items():
                if other_amp.position == amp.position and other_amp.depth < amp.depth:
                    continue

            
            # Check if we can just walk straight to the goal
            open_goal_position = -1
            goal_open = True
            goal_empty = True
            for other_key, other_amp in amps.items():
                if other_key is not key:
                    if other_amp.position == amp.goal:
                        if other_amp.depth == 1:
                            goal_open = False
                        goal_empty = False

            straight_walk_blocked = False
            if amp.position == amp.goal and amp.depth == 1:
                # special case to stop an amp moving to its own position
                straight_walk_blocked = True
            for pos in hallway_positions:
                for other_key, other_amp in amps.items():
                    if other_key is not key:
                        if amp.goal > amp.position:
                            # goal to the right
                            if other_amp.position < amp.goal and other_amp.position > amp.position and other_amp.depth == 0:
                                straight_walk_blocked = True
                        elif amp.goal < amp.position:
                            # goal to the left
                            if other_amp.position > amp.goal and other_amp.position < amp.position and other_amp.depth == 0:
                                straight_walk_blocked = True
            if not straight_walk_blocked:
                if goal_empty:
                    valid_positions.append((key, (amp.goal, 2)))
                elif goal_open:
                    go_to_goal = True
                    for other_key, other_amp in amps.items():
                        if other_key is not key:
                            if other_amp.position == amp.goal and other_amp.goal != amp.goal and other_amp.depth == 2:
                                go_to_goal = False
                    if go_to_goal:
                        valid_positions.append((key, (amp.goal, 1)))

            
            for pos in hallway_positions:
                blocked = False

                for other_key, other_amp in amps.items():
                    if other_key != key:
                        if pos < amp.position:
                            # Hallway positions to the left
                            if other_amp.position >= pos and other_amp.depth == 0:
                                blocked = True
                                break
                        elif pos > amp.position:
                            # Hallway positions to the right
                            if other_amp.position <= pos and other_amp.depth == 0:
                                blocked = True
                                break
                
                if not blocked:
                    valid_positions.append((key, (pos, 0)))
    
    return valid_positions

def solved(amps):
    for amp in amps.values():
        if amp.position != amp.goal:
            return False
    return True

def find_lowest_cost(amps, depth):
    if solved(amps):
        return 0

    lowest_cost = sys.maxsize

    if depth > 16:
        return lowest_cost

    valid_moves = find_valid_move_positions(amps)

    for move in valid_moves:
        move_cost = 0

        amp_to_move = amps[move[0]]

        hallway_positions = [0, 1, 3, 5, 7, 9, 10]

        if amp_to_move.move_count > 2:
            print("{} {} {}".format(amp_to_move.position, amp_to_move.depth, amp_to_move.goal))
            assert False
        original_position = amp_to_move.position
        original_depth = amp_to_move.depth

        move_cost += amp_to_move.distance_to_position(move[1][0], move[1][1])
        amp_to_move.change_position(move[1][0], move[1][1])
        amp_to_move.move_count += 1
        amp_to_move.moves.append(move[1][0])

        move_cost += find_lowest_cost(amps, depth + 1)

        amp_to_move.change_position(original_position, original_depth)
        amp_to_move.move_count -= 1
        amp_to_move.moves.pop()

        if move_cost < lowest_cost:
            lowest_cost = move_cost

    print("Current cost in branch at depth {} = {}".format(depth, lowest_cost))
        
    return lowest_cost

amps = {}
amps[1] = Amp(2, 1, 4, 10)
amps[2] = Amp(2, 2, 2, 1)
amps[3] = Amp(4, 1, 6, 100)
amps[4] = Amp(4, 2, 8, 1000)
amps[5] = Amp(6, 1, 4, 10)
amps[6] = Amp(6, 2, 6, 100)
amps[7] = Amp(8, 1, 8, 1000)
amps[8] = Amp(8, 2, 2, 1)

print(find_valid_move_positions(amps))

print(find_lowest_cost(amps, 1))

