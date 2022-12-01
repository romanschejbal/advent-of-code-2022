fn main() {
    let mut calories = include_str!("../input.txt")
        .split("\n\n")
        .into_iter()
        .map(|elf| {
            elf.lines()
                .map(str::parse::<usize>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
                .into_iter()
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    calories.sort();

    let max = *calories.last().unwrap();
    let top_three = calories.into_iter().rev().take(3).sum::<usize>();

    println!("Part 1: {max}\nPart 2: {top_three}");
}
