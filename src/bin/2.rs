use std::time::Instant;
const DAY_NUMBER: u32 = 2;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

enum WinCondition {
    Win,
    Lose,
    Draw
}

enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    pub fn value(&self) -> u32 {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn play(&self, other: &RockPaperScissors) -> u32 {
        let mut score = self.value();

        match *other {
            Self::Rock => {
                if let Self::Paper = self {
                    score += 6;
                }
                else if let Self::Rock = self {
                    score += 3
                }
            }
            Self::Paper => {
                if let Self::Scissors = self {
                    score += 6
                }
                else if let Self::Paper = self {
                    score += 3
                }
            }
            Self::Scissors => {
                if let Self::Rock = self {
                    score += 6
                }
                else if let Self::Scissors = self {
                    score += 3
                }
            }
        };

        score
    }
}

impl From<&str> for RockPaperScissors {
    fn from(source: &str) -> Self {
        match source {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => panic!("Invalid"),
        }
    }
}

impl From<(WinCondition, &RockPaperScissors)> for RockPaperScissors {
    fn from(source: (WinCondition, &RockPaperScissors)) -> Self {
        match source.1 {
            RockPaperScissors::Rock => match source.0 {
                WinCondition::Win => RockPaperScissors::Paper,
                WinCondition::Lose => RockPaperScissors::Scissors,
                WinCondition::Draw => RockPaperScissors::Rock,
            },
            RockPaperScissors::Paper => match source.0 {
                WinCondition::Win => RockPaperScissors::Scissors,
                WinCondition::Lose => RockPaperScissors::Rock,
                WinCondition::Draw => RockPaperScissors::Paper,
            },
            RockPaperScissors::Scissors => match source.0 {
                WinCondition::Win => RockPaperScissors::Rock,
                WinCondition::Lose => RockPaperScissors::Paper,
                WinCondition::Draw => RockPaperScissors::Scissors,
            },
        }
    }
}

impl From<&str> for WinCondition {
    fn from(source: &str) -> Self {
        match source {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid")
        }
    }
}

fn solve(input: impl Iterator<Item = String>, start: Instant) {
    let input = input.collect::<Vec<String>>();
    let sum = input.iter().map(|s| {
        let mut split = s.split_whitespace();
        (
            RockPaperScissors::from(split.next().unwrap()),
            RockPaperScissors::from(split.next().unwrap()),
        )
    }).fold(0u32, |acc, f| {
        let score = f.1.play(&f.0);
        acc + score
    });

    println!("{}", sum);
    println!("{}ns", start.elapsed().as_nanos());

    let sum = input.iter().map(|s| {
        let mut split = s.split_whitespace();
        let opponent = RockPaperScissors::from(split.next().unwrap());
        let selection = RockPaperScissors::from((WinCondition::from(split.next().unwrap()), &opponent));

        (
            opponent, selection
        )
    }).fold(0u32, |acc, f| {
        let score = f.1.play(&f.0);
        acc + score
    });

    

    println!("{}", sum);
    println!("{}ns", start.elapsed().as_nanos());
}
