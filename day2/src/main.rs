use day2::{EndState, Shape};

fn play(s: &str) -> usize {
    s.lines()
        .map(|l| {
            let mut it = l.split(' ').map(|s| s.parse::<Shape>().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .fold(0, |acc, (opponent, play)| {
            use EndState::*;
            use Shape::*;
            acc + play.to_score()
                + match (opponent, &play) {
                    (Rock, Paper) => Win,
                    (Rock, Rock) => Draw,
                    (Rock, Scissors) => Lose,
                    (Paper, Paper) => Draw,
                    (Paper, Rock) => Lose,
                    (Paper, Scissors) => Win,
                    (Scissors, Paper) => Lose,
                    (Scissors, Rock) => Win,
                    (Scissors, Scissors) => Draw,
                }
                .to_score()
        })
}

fn play2(s: &str) -> usize {
    s.lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(opponent, end_state)| {
                    (
                        opponent.parse::<Shape>().unwrap(),
                        end_state.parse::<EndState>().unwrap(),
                    )
                })
                .unwrap()
        })
        .fold(0, |acc, (opponent, end_state)| {
            use EndState::*;
            use Shape::*;
            acc + end_state.to_score()
                + match (opponent, &end_state) {
                    (Rock, Draw) => Rock,
                    (Rock, Win) => Paper,
                    (Rock, Lose) => Scissors,
                    (Paper, Draw) => Paper,
                    (Paper, Win) => Scissors,
                    (Paper, Lose) => Rock,
                    (Scissors, Draw) => Scissors,
                    (Scissors, Win) => Rock,
                    (Scissors, Lose) => Paper,
                }
                .to_score()
        })
}

fn main() {
    let input = include_str!("../input.txt");
    let score = play(input);
    println!("Part 1: {score}");
    let score = play2(input);
    println!("Part 2: {score}");

    let (part1, part2) = concise_solution(input);
    println!("Part 1: {part1}\nPart 2: {part2}");
}

fn concise_solution(input: &str) -> (usize, usize) {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .fold(
            (0, 0),
            |(part1, part2), (opponent, play_or_end_state)| match (opponent, play_or_end_state) {
                ("A", "X") => (part1 + 4, part2 + 3),
                ("A", "Y") => (part1 + 8, part2 + 4),
                ("A", "Z") => (part1 + 3, part2 + 8),
                ("B", "X") => (part1 + 1, part2 + 1),
                ("B", "Y") => (part1 + 5, part2 + 5),
                ("B", "Z") => (part1 + 9, part2 + 9),
                ("C", "X") => (part1 + 7, part2 + 2),
                ("C", "Y") => (part1 + 2, part2 + 6),
                ("C", "Z") => (part1 + 6, part2 + 7),
                pair => panic!("Uknown pair {pair:?}"),
            },
        )
}

#[test]
fn test() {
    assert_eq!(play("A Y\nB X\nC Z"), 15);
    assert_eq!(play2("A Y\nB X\nC Z"), 12);
    assert_eq!(concise_solution("A Y\nB X\nC Z"), (15, 12));
}
