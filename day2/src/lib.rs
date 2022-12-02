use std::str::FromStr;

pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl Shape {
    pub const fn to_score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

pub enum EndState {
    Lose,
    Draw,
    Win,
}

impl FromStr for EndState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

impl EndState {
    pub const fn to_score(&self) -> usize {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}
