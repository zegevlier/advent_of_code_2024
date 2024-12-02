use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut total = 0;

    for line in input.lines() {
        let mut line_split = line.split(' ');
        let mut prev_value = None;
        let mut goes_up = None;
        loop {
            let value: i32 = match line_split.next() {
                Some(v) => v.parse().unwrap(),
                None => {
                    total += 1;
                    break;
                }
            };
            if let Some(pv) = prev_value {
                // We have a previous value, so we need to ensure it's in range. Let's first ensure it goes up and down as expected.
                match goes_up {
                    Some(gu) => {
                        if gu == (pv > value) {
                            break;
                        }
                    }
                    None => goes_up = Some(pv < value),
                }
                // Now we check if it's in range
                let diff = ((pv - value) as i32).abs();
                if !(1..=3).contains(&diff) {
                    break;
                }
            };
            prev_value = Some(value)
        }
    }

    total
}

fn verify_line(line: Vec<&str>, grace_given: bool) -> bool {
    let mut prev_value: Option<i32> = None;
    let mut goes_up = None;

    let mut line_iter = line.iter();
    let mut i = 0;
    loop {
        let value: i32 = match line_iter.next() {
            Some(v) => v.parse().unwrap(),
            None => {
                // We're at the end of the list. This one is valid.
                return true;
            }
        };
        if let Some(pv) = prev_value {
            // We have a previous value, so we need to ensure it's in range. Let's first ensure it goes up and down as expected.
            if let Some(gu) = goes_up {
                if gu == (pv > value) {
                    if !grace_given {
                        // We need to re-check this list with the value we got stuck on removed.
                        // Note, we may also need to remove the value just before the one where we got stuck.

                        // This is painful, but I don't have time to fix it today. Might try later!
                        let new_line = [line[0..i].to_vec(), line[i + 1..].to_vec()].concat();
                        if verify_line(new_line, true) {
                            return true;
                        }
                        let new_line = [line[0..i - 1].to_vec(), line[i..].to_vec()].concat();
                        if verify_line(new_line, true) {
                            return true;
                        }
                        let new_line = [line[0..i - 2].to_vec(), line[i-1..].to_vec()].concat();
                        return verify_line(new_line, true);
                    }
                    return false;
                }
            } else {
                goes_up = Some(pv < value)
            }
            // Now we check if it's in range
            let diff = (pv - value).abs();
            if !(1..=3).contains(&diff) {
                if !grace_given {
                    // We need to re-check this list with the value we got stuck on removed.
                    // Note, we may also need to remove the value just before the one where we got stuck.
                    let new_line = [line[0..i].to_vec(), line[i + 1..].to_vec()].concat();
                    if verify_line(new_line, true) {
                        return true;
                    }
                    let new_line = [line[0..i - 1].to_vec(), line[i..].to_vec()].concat();
                    return verify_line(new_line, true);
                }
                return false;
            }
        };
        prev_value = Some(value);
        i += 1;
    }
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let mut total = 0;

    for line in input.lines() {
        let line_split = line.split(' ').collect();

        if verify_line(line_split, false) {
            total += 1;
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
5 4 6 7 8
1 10 11 12 13
9 1 2 3 4";

    // 💖 https://www.reddit.com/r/adventofcode/comments/1h4shdu/2024_day_2_part2_edge_case_finder/
    const EDGECASE_FROM_REDDIT: &str = "48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7
7 10 8 10 11
29 28 27 25 26 25 22 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 7);
    }

    #[test]
    fn test_part2_edgecases() {
        assert_eq!(part2(EDGECASE_FROM_REDDIT), 10);
    }
}
