use std::collections::{ HashMap, HashSet };

// true = obstacle, false = empty
type Map = Vec<Vec<bool>>;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

struct Guard {
    xpos: usize,
    ypos: usize,
    // current direction the guard is facing
    direction: GuardDirection,
    // locations visited by the guard
    visited: HashMap<(usize, usize), HashSet<GuardDirection>>,
}

impl Guard {
    fn new(xpos: usize, ypos: usize) -> Self {
        let mut visited = HashMap::new();
        visited.insert((xpos, ypos), HashSet::from([GuardDirection::Up]));

        Guard {
            xpos,
            ypos,
            direction: GuardDirection::Up,
            visited,
        }
    }

    // the next space in the direction the guard is facing (x, y)
    // returns None if either coordinate goes below 0 (out of bounds)
    fn next_pos(&self) -> Option<(usize, usize)> {
        match self.direction {
            GuardDirection::Up => {
                if let Some(new_ypos) = usize::checked_sub(self.ypos, 1) {
                    Some((self.xpos, new_ypos))
                } else {
                    None
                }
            }
            GuardDirection::Down => Some((self.xpos, self.ypos + 1)),
            GuardDirection::Left => {
                if let Some(new_xpos) = usize::checked_sub(self.xpos, 1) {
                    Some((new_xpos, self.ypos))
                } else {
                    None
                }
            }
            GuardDirection::Right => Some((self.xpos + 1, self.ypos)),
        }
    }

    // progresses the guard's position, and returns true if the guard has already
    // visited this location in the current direction
    fn step(&mut self) -> bool {
        let Some((xpos, ypos)) = self.next_pos() else {
            return false;
        };
        self.xpos = xpos;
        self.ypos = ypos;
        
        if let Some(pos) = self.visited.get_mut(&(xpos, ypos)) {
            !pos.insert(self.direction)
        } else {
            self.visited.insert((xpos, ypos), HashSet::from([self.direction]));
            false
        }
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            GuardDirection::Up => GuardDirection::Right,
            GuardDirection::Down => GuardDirection::Left,
            GuardDirection::Left => GuardDirection::Up,
            GuardDirection::Right => GuardDirection::Down,
        };
    }
}

fn parse_input(input: &str) -> Map {
    input.lines()
        .map(|line|
            line.chars().map(|c| c == '#').collect()
        )
        .collect()
}

fn find_guard(input: &str, rows: usize, cols: usize) -> (usize, usize) {
    let mut guard_pos = input.find('^').unwrap();
    guard_pos -= guard_pos / (cols + 1);

    let ypos = guard_pos / cols;
    let xpos = guard_pos % rows;

    (xpos, ypos)
}

/******************************************************************************
 *                                  PROBLEM 1                                 *
 ******************************************************************************/

pub fn prob6_1(input: &str) {
    let map = parse_input(input);

    let mut guard = {
        let guard_pos = find_guard(input, map[0].len(), map.len());
        Guard::new(guard_pos.0, guard_pos.1)
    };

    track_guard(&map, &mut guard);

    println!("[6:1] Unique positions visited: {}", guard.visited.len());
}

fn track_guard(map: &Map, guard: &mut Guard) {
    loop {
        let Some((xpos, ypos)) = guard.next_pos() else {
            break;
        };

        if map.len() <= ypos || map[ypos].len() <= xpos {
            break;
        }

        if map[ypos][xpos] {
            guard.turn();
        } else {
            guard.step();
        }
    }
}

/******************************************************************************
 *                                  PROBLEM 2                                 *
 ******************************************************************************/

pub fn prob6_2(input: &str) {
    let mut map = parse_input(input);

    let guard_pos = find_guard(input, map[0].len(), map.len());
    let mut guard = Guard::new(guard_pos.0, guard_pos.1);

    track_guard(&map, &mut guard);

    let visited = guard.visited
        .iter()
        .map(|((x, y), _directions)| (*x, *y))
        .collect::<HashSet<(usize, usize)>>();

    let candidates = get_obstacle_candidates(&visited);
    let loops = find_loops(&mut map, &candidates, guard_pos);

    println!("[6:2] Successful obstacle positions: {}", loops);
}

fn get_obstacle_candidates(
    visited: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut candidates = HashSet::new();

    for point in visited {
        candidates.insert(*point);
        // down:
        candidates.insert((point.0, point.1 + 1));
        // right:
        candidates.insert((point.0 + 1, point.1));
        // up:
        if let Some(new_ypos) = usize::checked_sub(point.1, 1) {
            candidates.insert((point.0, new_ypos));
        }
        // left:
        if let Some(new_xpos) = usize::checked_sub(point.0, 1) {
            candidates.insert((new_xpos, point.1));
        }
    }

    candidates
}

fn find_loops(
    map: &mut Map,
    candidates: &HashSet<(usize, usize)>,
    guard_pos: (usize, usize),
) -> usize {
    let mut loop_count = 0;

    for point in candidates {
        let (xpos, ypos) = *point;
        if map.len() <= ypos
        || map[ypos].len() <= xpos
        || map[ypos][xpos] 
        || (xpos == guard_pos.0 && ypos == guard_pos.1) {
            continue;
        }

        map[ypos][xpos] = true;

        let mut guard = Guard::new(guard_pos.0, guard_pos.1);
        
        if check_with_obstacles(map, &mut guard) {
            loop_count += 1;
        }

        map[ypos][xpos] = false;
    }

    loop_count
}

// check if a guard enters an infinite loop on a given map
fn check_with_obstacles(map: &Map, guard: &mut Guard) -> bool {
    loop {
        let Some((xpos, ypos)) = guard.next_pos() else {
            return false;
        };

        if map.len() <= ypos || map[ypos].len() <= xpos {
            return false;
        }

        if map[ypos][xpos] {
            guard.turn();
        } else {
            if guard.step() {
                return true;
            }
        }
    }
}

/******************************************************************************/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prob6_1_guardpos() {
        let input = &std::fs::read_to_string("input/6_1_example.txt").unwrap();
        let map = parse_input(input);
        let guard_pos = find_guard(input, map[0].len(), map.len());
        assert_eq!(guard_pos, (4, 6));
    }

    #[test]
    fn prob6_1_example() {
        let input = &std::fs::read_to_string("input/6_1_example.txt").unwrap();
        let map = parse_input(input);
        let guard_pos = find_guard(input, map[0].len(), map.len());
        let mut guard = Guard::new(guard_pos.0, guard_pos.1);

        track_guard(&map, &mut guard);

        assert_eq!(guard.visited.len(), 41);
    }
}
