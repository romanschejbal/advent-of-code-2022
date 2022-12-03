use std::collections::{HashMap, HashSet};

fn score(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - 96
    } else {
        (c as u32) - 38
    }
}

fn main() {
    let backpacks = include_str!("../input.txt");
    let part1 = backpacks.lines().fold(0, |acc, line| {
        let mut m = line[..line.len() / 2]
            .chars()
            .map(|c| (c, 1))
            .collect::<HashMap<_, _>>();
        acc + line[line.len() / 2..]
            .chars()
            .filter(|c| {
                if let Some(s) = m.get_mut(c) {
                    if *s == 1 {
                        *s += 1;
                        return true;
                    }
                }
                false
            })
            .map(score)
            .sum::<u32>()
    });
    println!("Part 1: {part1}");

    let part2 = backpacks
        .lines()
        .zip(backpacks.lines().skip(1))
        .zip(backpacks.lines().skip(2))
        .step_by(3)
        .fold(0, |acc, ((l1, l2), l3)| {
            acc + l1
                .chars()
                .filter(|c| l2.contains(*c) && l3.contains(*c))
                .map(score)
                .collect::<HashSet<_>>()
                .into_iter()
                .sum::<u32>()
        });

    println!("Part 2: {part2}");
}
