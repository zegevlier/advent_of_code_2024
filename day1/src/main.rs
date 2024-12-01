fn part_one(input: &str) -> i32 {
    let mut l1: Vec<i32> = vec![];
    let mut l2: Vec<i32> = vec![];

    input.lines().for_each(|line| {
        let mut line_split = line.split("   ");
        l1.push(line_split.next().unwrap().parse().unwrap());
        l2.push(line_split.next().unwrap().parse().unwrap());
    });

    l1.sort();
    l2.sort();

    let mut total_diff = 0;
    for i in 0..l1.len() {
        let diff = (l1[i] - l2[i]).abs();
        total_diff += diff;
    }
    total_diff
}

fn part_two(input: &str) -> i32 {
    let mut l1: Vec<i32> = vec![];
    let mut l2: Vec<i32> = vec![];

    input.lines().for_each(|line| {
        let mut line_split = line.split("   ");
        l1.push(line_split.next().unwrap().parse().unwrap());
        l2.push(line_split.next().unwrap().parse().unwrap());
    });

    l1.iter()
        .map(|v| v * l2.iter().filter(|r| *r == v).count() as i32)
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}
