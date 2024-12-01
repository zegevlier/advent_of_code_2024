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
pub fn part2(input: &str) -> usize {
    let lines = input.lines();
    // We can't get the size without consuming the iterator, which would be very slow.
    // So, we just initialise it at 1k and hope that's roughly correct.
    let mut l1: Vec<usize> = Vec::with_capacity(1000);
    let mut l2: Vec<usize> = Vec::with_capacity(1000);

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

    let mut total = 0;
    let mut previous_value = 0;
    let mut previous_s = 0;
    for value in l1 {
        if value == previous_value {
            total += previous_s;
            continue;
        }
        previous_value = value;
        // We find the value in the other list using binary search
        let low = l2.partition_point(|x| x < &value);
        let high = l2.partition_point(|x| x <= &value);
        previous_s = value * (high - low);
        total += previous_s;
    }

    total
}
