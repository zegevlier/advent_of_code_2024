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
                if diff > 3 || diff < 1 {
                    break;
                }
            };
            prev_value = Some(value)
        }
    }

    total
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let mut total = 0;

    for line in input.lines() {
        let mut line_split = line.split(' ').peekable();
        let mut prev_value = None;
        let mut goes_up: Option<bool> = None;
        let mut grace_given = false;
        let mut first_value = None;
        let mut i = 0;
        loop {
            let value: i32 = match line_split.next() {
                Some(v) => v.parse().unwrap(),
                None => {
                    total += 1;
                    break;
                }
            };
            if let Some(mut pv) = prev_value {
                // We have a previous value, so we need to ensure it's in range. Let's first ensure it goes up and down as expected.
                match goes_up {
                    Some(gu) => {
                        if gu == (pv > value) {
                            if grace_given {
                                break;
                            } else {
                                grace_given = true;
                                // If this is not the third element in the list, we just remove it and see if it works now.
                                if i != 2 {
                                    continue;
                                }
                                // If this is the third element in the list, we could either remove the second or the third one.
                                // We can determine whether we need to do this by looking at the next available value,
                                // if it goes the same direction as the second, we remove the third. If it goes the same direction
                                // as the third, we remove the second.
                                let next_value = line_split.peek().unwrap().parse::<i32>().unwrap();
                                // first_value is for 0
                                // pv is for 1
                                // value is for 2
                                // next_value is for 3
                                // gu is for 0 compared to 1
                                // gu != first_value < value
                                if gu == (first_value.unwrap() < next_value) {
                                    // We need to remove 2, since the next one lines up with the one after.
                                    // Just continuing here does this.
                                    continue;
                                } else {
                                    // We need to remove 1. This means resetting the pv to the first value, and updating the goes up.
                                    goes_up = Some(!gu);
                                    prev_value = first_value;
                                    pv = first_value.unwrap();
                                    // We need to do the rest of the checks here.
                                }
                            }
                        }
                    }
                    None => goes_up = Some(pv < value),
                }
                // Now we check if it's in range
                let diff = ((pv - value) as i32).abs();
                if diff > 3 || diff < 1 {
                    if grace_given {
                        break;
                    } else {
                        grace_given = true;
                        if i != 1 {
                            continue;
                        }
                        goes_up = None;
                        // We might need to remove the first one instead of the second one. 
                        let next_value = line_split.peek().unwrap().parse::<i32>().unwrap();
                        // If the next value is in range compared to the current value, we remove the first value.
                        // No need to update first_value, because we've already given grace here.
                        let diff = ((value - next_value) as i32).abs();
                        if diff > 3 || diff < 1 {
                            // We remove the second value instead.
                            continue;
                        } else {
                            // We need to remove the first value
                            // We do this by allowing it to set the prev value.
                        }
                    }
                }
            } else {
                first_value = Some(value);
            };
            prev_value = Some(value);
            i += 1;
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 7);
    }
}
