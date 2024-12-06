use std::{
    fmt::{Display, Write},
    vec,
};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Empty,
    Obstacle,
    Guard(Direction),
    Visited,
}

impl Square {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Obstacle,
            '^' => Self::Guard(Direction::Up),
            'v' => Self::Guard(Direction::Down),
            '<' => Self::Guard(Direction::Left),
            '>' => Self::Guard(Direction::Right),
            'X' => Self::Visited,
            _ => panic!("Invalid character"),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Square::Empty => '.',
            Square::Obstacle => '#',
            Square::Guard(direction) => match direction {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            },
            Square::Visited => 'X',
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_offsets(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid = vec![];

    let mut guard_pos = (0, 0);
    let mut guard_direction = Direction::Up;
    for (i, line) in input.lines().enumerate() {
        let mut grid_line = vec![];
        for (j, c) in line.chars().enumerate() {
            let grid_char = Square::from_char(c);
            if let Square::Guard(dir) = grid_char {
                guard_direction = dir;
                guard_pos = (i, j)
            };
            grid_line.push(grid_char);
        }
        grid.push(grid_line);
    }

    loop {
        // We first check if we can go into the direction the guard is facing.
        let offset = guard_direction.get_offsets();
        // If we would run out of the map, set the current position to visited and break from the loop.
        let new_pos = (
            guard_pos.0 as isize + offset.0,
            guard_pos.1 as isize + offset.1,
        );

        if new_pos.0 >= grid.len() as isize
            || new_pos.1 >= grid[0].len() as isize
            || new_pos.0 < 0
            || new_pos.1 < 0
        {
            grid[guard_pos.0][guard_pos.1] = Square::Visited;
            break;
        }

        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);

        match grid[new_pos.0][new_pos.1] {
            Square::Empty | Square::Visited => {
                grid[guard_pos.0][guard_pos.1] = Square::Visited;
                guard_pos = new_pos;
            }
            Square::Obstacle => {
                // Turn 90 degrees right and try again.
                guard_direction = guard_direction.turn_right();
            }
            _ => {}
        }
    }

    grid.iter()
        .flatten()
        .filter(|c| **c == Square::Visited)
        .count()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i32 {
    let mut grid = vec![];

    let mut guard_pos = (0, 0);
    let mut guard_direction = Direction::Up;
    for (i, line) in input.lines().enumerate() {
        let mut grid_line = vec![];
        for (j, c) in line.chars().enumerate() {
            let grid_char = Square::from_char(c);
            if let Square::Guard(dir) = grid_char {
                guard_direction = dir;
                guard_pos = (i, j)
            };
            grid_line.push(grid_char);
        }
        grid.push(grid_line);
    }

    let guard_starting_pos = guard_pos;
    let guard_starting_direction = guard_direction;
    let starting_grid = grid.clone();

    loop {
        // We first check if we can go into the direction the guard is facing.
        let offset = guard_direction.get_offsets();
        // If we would run out of the map, set the current position to visited and break from the loop.
        let new_pos = (
            guard_pos.0 as isize + offset.0,
            guard_pos.1 as isize + offset.1,
        );

        if new_pos.0 >= grid.len() as isize
            || new_pos.1 >= grid[0].len() as isize
            || new_pos.0 < 0
            || new_pos.1 < 0
        {
            grid[guard_pos.0][guard_pos.1] = Square::Visited;
            break;
        }

        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);

        match grid[new_pos.0][new_pos.1] {
            Square::Empty | Square::Visited => {
                grid[guard_pos.0][guard_pos.1] = Square::Visited;
                guard_pos = new_pos;
            }
            Square::Obstacle => {
                // Turn 90 degrees right and try again.
                guard_direction = guard_direction.turn_right();
            }
            _ => {}
        }
    }

    let mut total = 0;

    // We now have a visited grid. For every point that is visited, we try to put an obstacle there, and see if that puts our dear friend into a loop.
    for (i, line) in grid.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            if *cell != Square::Visited {
                // We don't have to bother exploring this one.
                continue;
            }
            if (i, j) == guard_starting_pos {
                continue;
            }

            // We try to put an obstacle there, and see if we get stuck in a loop.
            let mut grid = starting_grid.clone();
            grid[i][j] = Square::Obstacle;

            let mut guard_pos = guard_starting_pos;
            let mut guard_direction = guard_starting_direction;

            let mut visited_places = vec![];

            loop {
                if visited_places.contains(&(guard_pos, guard_direction)) {
                    total += 1;
                    break;
                }

                visited_places.push((guard_pos, guard_direction));
                // We first check if we can go into the direction the guard is facing.
                let offset = guard_direction.get_offsets();
                // If we would run out of the map, set the current position to visited and break from the loop.
                let new_pos = (
                    guard_pos.0 as isize + offset.0,
                    guard_pos.1 as isize + offset.1,
                );

                if new_pos.0 >= grid.len() as isize
                    || new_pos.1 >= grid[0].len() as isize
                    || new_pos.0 < 0
                    || new_pos.1 < 0
                {
                    grid[guard_pos.0][guard_pos.1] = Square::Visited;
                    // We didn't get in a loop.
                    break;
                }

                let new_pos = (new_pos.0 as usize, new_pos.1 as usize);

                match grid[new_pos.0][new_pos.1] {
                    Square::Empty | Square::Visited => {
                        grid[guard_pos.0][guard_pos.1] = Square::Visited;
                        guard_pos = new_pos;
                    }
                    Square::Obstacle => {
                        // Turn 90 degrees right and try again.
                        guard_direction = guard_direction.turn_right();
                    }
                    _ => {}
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 6);
    }
}
