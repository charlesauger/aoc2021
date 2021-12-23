import sys

class Amp:
    def __init__(self, position, depth, goal, cost):
        self.position = position
        self.depth = depth
        self.goal = goal
        self.cost = cost

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

def find_lowest_cost(amps):
    if solved(amps):
        return 0
    
    lowest_cost = sys.maxsize

    valid_moves = find_valid_move_positions(amps)

    print("Number of valid moves = {}".format(len(valid_moves)))

    for move in valid_moves:
        move_cost = 0

        amp_to_move = amps[move[0]]
        original_position = amp_to_move.position
        original_depth = amp_to_move.depth

        move_cost += amp_to_move.distance_to_position(move[1][0], move[1][1])
        amp_to_move.change_position(move[1][0], move[1][1])

        move_cost += find_lowest_cost(amps)

        amp_to_move.change_position(original_position, original_depth)

        if move_cost < lowest_cost:
            lowest_cost = move_cost

    # print("Current cost in branch = {}".format(lowest_cost))
        
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

print(find_lowest_cost(amps))

