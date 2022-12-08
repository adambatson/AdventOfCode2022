use std::str::FromStr;

const INPUT: &'static str = include_str!("../../inputs/day02.txt");

enum RockPaperScissorsShape {
    Rock,
    Paper,
    Scissors
}

impl FromStr for RockPaperScissorsShape {
    type Err = ();

    fn from_str(input: &str) -> Result<RockPaperScissorsShape, Self::Err> {
        match input {
            "A" | "X"  => Ok(RockPaperScissorsShape::Rock),
            "B" | "Y"  => Ok(RockPaperScissorsShape::Paper),
            "C" | "Z"  => Ok(RockPaperScissorsShape::Scissors),
            _          => Err(())
        }
    }
}

impl RockPaperScissorsShape {
    pub fn does_win(self, other: RockPaperScissorsShape) -> RockPaperScissorsResult{
        let res = match self {
            RockPaperScissorsShape::Rock => {
                match other {
                    RockPaperScissorsShape::Rock => RockPaperScissorsResult::Draw,
                    RockPaperScissorsShape::Paper => RockPaperScissorsResult::Loss,
                    RockPaperScissorsShape::Scissors => RockPaperScissorsResult::Win
                }
            },
            RockPaperScissorsShape::Paper => {
                match other {
                    RockPaperScissorsShape::Rock => RockPaperScissorsResult::Win,
                    RockPaperScissorsShape::Paper => RockPaperScissorsResult::Draw,
                    RockPaperScissorsShape::Scissors => RockPaperScissorsResult::Loss
                }
            },
            RockPaperScissorsShape::Scissors => {
                match other {
                    RockPaperScissorsShape::Rock => RockPaperScissorsResult::Loss,
                    RockPaperScissorsShape::Paper => RockPaperScissorsResult::Win,
                    RockPaperScissorsShape::Scissors => RockPaperScissorsResult::Draw
                }
            }
        };
        return res;
    }
}

enum RockPaperScissorsResult {
    Win,
    Loss,
    Draw
}

pub fn solve() {
    println!("Solving Day 02!\n");
}