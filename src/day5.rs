use std::{borrow::Borrow, collections::HashSet, vec};

use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    let mut lines = input.lines();

    let mut rules = Vec::new();
    for line in &mut lines {
        if line.is_empty() {
            // We've gotten all the rules, now the input.
            break;
        }

        let (node1, node2) = line.split_once('|').unwrap();
        let (node1, node2): (u8, u8) = (node1.parse().unwrap(), node2.parse().unwrap());

        rules.push((node1, node2));
    }

    let mut total = 0;
    for line in lines {
        // These are the input.
        let elements: Vec<u8> = line.split(',').map(|n| n.parse::<u8>().unwrap()).collect();

        let elements_set: HashSet<&u8> = HashSet::from_iter(elements.iter());

        let relevant_rules = rules
            .iter()
            .filter(|(i, o)| elements_set.contains(i) && elements_set.contains(o))
            .collect::<Vec<_>>();

        let mut failed = false;
        for (first, last) in relevant_rules {
            let index_first = elements.iter().position(|e| e == first).unwrap();
            let index_last = elements.iter().position(|e| e == last).unwrap();
            if index_first > index_last {
                // BAD!
                failed = true;
                break;
            }
        }
        if !failed {
            total += *elements.get(elements.len() / 2).unwrap() as i32;
        }
    }

    total
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    let mut lines = input.lines();

    let mut rules = Vec::new();
    for line in &mut lines {
        if line.is_empty() {
            // We've gotten all the rules, now the input.
            break;
        }

        let (node1, node2) = line.split_once('|').unwrap();
        let (node1, node2): (u8, u8) = (node1.parse().unwrap(), node2.parse().unwrap());

        rules.push((node1, node2));
    }

    let mut total = 0;
    for line in lines {
        // These are the input.
        let elements: Vec<u8> = line.split(',').map(|n| n.parse::<u8>().unwrap()).collect();

        let mut elements_set: HashSet<&u8> = HashSet::from_iter(elements.iter());

        let mut relevant_rules = rules
            .iter()
            .filter(|(i, o)| elements_set.contains(i) && elements_set.contains(o))
            .collect::<Vec<_>>();

        let mut failed = false;
        for (first, last) in relevant_rules.iter() {
            let index_first = elements.iter().position(|e| e == first).unwrap();
            let index_last = elements.iter().position(|e| e == last).unwrap();
            if index_first > index_last {
                // BAD!
                failed = true;
                break;
            }
        }
        if !failed {
            // We ignore those for this one.
            continue;
        }

        let mut sorted_order = Vec::new();

        // This one has failed. We already know all the relevant rules and nodes.
        // Now we do everyone's favourite Kahn's algorithm for topological sorting (finally a use for all those lectures lol)
        let mut no_incoming_edges_nodes = elements
            .iter()
            .filter(|e| !relevant_rules.iter().any(|(_, o)| &o == e))
            .collect::<Vec<_>>();
        while let Some(n) = no_incoming_edges_nodes.pop() {
            sorted_order.push(n);

            let mut to_check = Vec::new();
            relevant_rules.retain(|(i, o)| {
                if i == n {
                    to_check.push(o);
                    false
                } else {
                    true
                }
            });

            no_incoming_edges_nodes.extend(
                to_check
                    .iter()
                    .filter(|e| !relevant_rules.iter().any(|(_, o)| &o == *e)),
            )
        }

        total += **sorted_order.get(sorted_order.len() / 2).unwrap() as i32;
    }

    total
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 123);
    }
}
