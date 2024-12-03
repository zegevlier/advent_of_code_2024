use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    regex
        .captures_iter(input)
        .map(|capture| {
            let a: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
            let b: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
            a * b
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let regex = Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\))|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    regex
        .captures_iter(input)
        .map(|capture| {
            let full_str = capture.get(0).unwrap().as_str();
            match full_str {
                "do()" => {
                    enabled = true;
                    0
                }
                "don't()" => {
                    enabled = false;
                    0
                }
                _ => {
                    if !enabled {
                        0
                    } else {
                        let a: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
                        let b: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
                        a * b
                    }
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT2), 48);
    }
}
