use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let lines = input.lines();
    // We can't get the size without consuming the iterator, which would be very slow.
    // So, we just initialise it at 1k and hope that's roughly correct.
    let mut l1: Vec<i32> = Vec::with_capacity(1000);
    let mut l2: Vec<i32> = Vec::with_capacity(1000);

    for line in lines {
        // Splitting on a char is *much* faster than splitting on a string. We know the separator is 3 spaces, so we just ignore the other two.
        // If we know for sure the input is always 5 chars longs, we can make this parsing even faster, but we don't.
        let line_split = line.split_once(' ').unwrap();
        l1.push(line_split.0.parse().unwrap());
        l2.push(line_split.1[2..].parse().unwrap());
    }

    // There might be a faster way of sorting these, but this is decently fast.
    l1.sort();
    l2.sort();

    // This was the fastest way I found to implement this. There are probably faster ways.
    l1.iter()
        .zip(l2.iter())
        .map(|(l, r)| if l < r { r - l } else { l - r })
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
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
