use aoc_runner_derive::aoc;

const WORD: &str = "XMAS";

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let letter_grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut total = 0;
    for x in 0..letter_grid.len() {
        for y in 0..letter_grid[0].len() {
            if letter_grid[x][y] == 'X' {
                total += check_for_xmas(&letter_grid, x, y);
            }
        }
    }
    total
}

fn check_for_xmas(letter_grid: &[Vec<char>], x: usize, y: usize) -> i32 {
    // We've just found the word_idx'th character in the WORD.

    let mut total = 0;
    // Now we look in the entire grid around the current index (x,y).

    for x_offset in -1..=1 {
        for y_offset in -1..=1 {
            if x_offset == 0 && y_offset == 0 {
                // This is the current value we're looking at, no need to recheck.
                continue;
            }
            if recursive_find_word(letter_grid, x, y, (x_offset, y_offset), 1) {
                total += 1;
            }
        }
    }

    total
}

fn recursive_find_word(
    letter_grid: &[Vec<char>],
    x: usize,
    y: usize,
    offset: (i32, i32),
    word_idx: usize,
) -> bool {
    if !offset_valid(x, y, offset.0, offset.1, letter_grid) {
        // Following this word would go out of range.
        return false;
    }

    let next_char_to_find = WORD.chars().nth(word_idx).unwrap();
    let new_x = (x as i32 + offset.0) as usize;
    let new_y = (y as i32 + offset.1) as usize;
    if letter_grid[new_x][new_y] == next_char_to_find {
        if word_idx == WORD.len() - 1 {
            return true;
        }
        return recursive_find_word(letter_grid, new_x, new_y, offset, word_idx + 1);
    }
    false
}

fn offset_valid(
    x: usize,
    y: usize,
    x_offset: i32,
    y_offset: i32,
    letter_grid: &[Vec<char>],
) -> bool {
    if (y_offset == -1 && y == 0) || (x_offset == -1 && x == 0) {
        // Don't underflow
        return false;
    }
    if (y_offset == 1 && y == letter_grid.len() - 1)
        || (x_offset == 1 && x == letter_grid[0].len() - 1)
    {
        // Don't overflow
        return false;
    }
    true
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let letter_grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut total = 0;
    // We don't need to check the first and last row, because they can never contain an X-MAS.
    for x in 1..letter_grid.len() - 1 {
        for y in 1..letter_grid[0].len() - 1 {
            if letter_grid[x][y] == 'A' && check_for_x_mas(&letter_grid, x, y) {
                total += 1
            }
        }
    }
    total
}

fn check_for_x_mas(letter_grid: &[Vec<char>], x: usize, y: usize) -> bool {
    let top_left = letter_grid[x - 1][y - 1];
    let top_right = letter_grid[x - 1][y + 1];
    let bottom_left = letter_grid[x + 1][y - 1];
    let bottom_right = letter_grid[x + 1][y + 1];

    if top_right == 'M' && bottom_left == 'S' {
        if top_left == 'M' && bottom_right == 'S' {
            true
        } else {
            top_left == 'S' && bottom_right == 'M'
        }
    } else if top_right == 'S' && bottom_left == 'M' {
        if top_left == 'M' && bottom_right == 'S' {
            return true;
        } else {
            return top_left == 'S' && bottom_right == 'M';
        }
    } else {
        return false;
    }
}

#[cfg(test)]
mod test {
    use crate::day4::part2;

    use super::part1;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 9);
    }
}
