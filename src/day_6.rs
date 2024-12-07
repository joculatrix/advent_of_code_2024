use std::collections::HashSet;

// true = obstacle, false = empty
type Map = Vec<Vec<bool>>;

#[derive(Copy, Clone)]
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
    visited: HashSet<(usize, usize)>,
}

impl Guard {
    fn new(xpos: usize, ypos: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert((xpos, ypos));

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

    fn step(&mut self) {
        let Some((xpos, ypos)) = self.next_pos() else {
            return;
        };
        self.xpos = xpos;
        self.ypos = ypos;
        self.visited.insert((xpos, ypos));
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

pub fn prob6_1(input: &str) {
    let map = parse_input(input);

    let mut guard = {
        let guard_pos = find_guard(input, map[0].len(), map.len());
        Guard::new(guard_pos.0, guard_pos.1)
    };

    track_guard(map, &mut guard);

    println!("[6:1] Unique positions visited: {}", guard.visited.len());
}

fn track_guard(map: Map, guard: &mut Guard) {
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

        track_guard(map, &mut guard);

        assert_eq!(guard.visited.len(), 41);
    }
}
