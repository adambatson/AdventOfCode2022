use std::str::FromStr;

const INPUT: &'static str = include_str!("../../inputs/day02.txt");

trait Score {
    fn score(&self) -> u64;
}

#[derive(Debug, Clone, Copy)]
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

impl Score for RockPaperScissorsShape {
    fn score(&self) -> u64 {
        match self {
            RockPaperScissorsShape::Rock => 1,
            RockPaperScissorsShape::Paper => 2,
            RockPaperScissorsShape::Scissors => 3
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RockPaperScissorsResult {
    Win,
    Loss,
    Draw
}

impl Score for RockPaperScissorsResult {
    fn score(&self) -> u64 {
        match self {
            RockPaperScissorsResult::Win => 6,
            RockPaperScissorsResult::Loss => 0,
            RockPaperScissorsResult::Draw => 3
        }
    }
}

impl FromStr for RockPaperScissorsResult {
    type Err = ();
    fn from_str(input: &str) -> Result<RockPaperScissorsResult, Self::Err> {
        match input {
            "X"  => Ok(RockPaperScissorsResult::Loss),
            "Y"  => Ok(RockPaperScissorsResult::Draw),
            "Z"  => Ok(RockPaperScissorsResult::Win),
            _          => Err(())
        }
    }
}

#[derive(Clone, Copy)]
struct RockPaperScissorsRound {
    me:  Option<RockPaperScissorsShape>,
    opponent : RockPaperScissorsShape,
    result: Option<RockPaperScissorsResult>
}

impl RockPaperScissorsRound {
    pub fn play_round(&mut self) {
        let res = match(&self.me, &self.opponent) {
            (Some(RockPaperScissorsShape::Rock), RockPaperScissorsShape::Paper) => RockPaperScissorsResult::Loss,
            (Some(RockPaperScissorsShape::Rock), RockPaperScissorsShape::Scissors) => RockPaperScissorsResult::Win,
            (Some(RockPaperScissorsShape::Paper), RockPaperScissorsShape::Rock) => RockPaperScissorsResult::Win,
            (Some(RockPaperScissorsShape::Paper), RockPaperScissorsShape::Scissors) => RockPaperScissorsResult::Loss,
            (Some(RockPaperScissorsShape::Scissors), RockPaperScissorsShape::Paper) => RockPaperScissorsResult::Win,
            (Some(RockPaperScissorsShape::Scissors), RockPaperScissorsShape::Rock) => RockPaperScissorsResult::Loss,
            _ => RockPaperScissorsResult::Draw
        };
        self.result = Some(res);
    }

    pub fn determine_shape(&mut self) {
        let shape = match(&self.opponent, &self.result.unwrap()) {
            (RockPaperScissorsShape::Rock, RockPaperScissorsResult::Win) => RockPaperScissorsShape::Paper,
            (RockPaperScissorsShape::Rock, RockPaperScissorsResult::Draw) => RockPaperScissorsShape::Rock,
            (RockPaperScissorsShape::Rock, RockPaperScissorsResult::Loss) => RockPaperScissorsShape::Scissors,
            (RockPaperScissorsShape::Paper, RockPaperScissorsResult::Win) => RockPaperScissorsShape::Scissors,
            (RockPaperScissorsShape::Paper, RockPaperScissorsResult::Draw) => RockPaperScissorsShape::Paper,
            (RockPaperScissorsShape::Paper, RockPaperScissorsResult::Loss) => RockPaperScissorsShape::Rock,
            (RockPaperScissorsShape::Scissors, RockPaperScissorsResult::Win) => RockPaperScissorsShape::Rock,
            (RockPaperScissorsShape::Scissors, RockPaperScissorsResult::Draw) => RockPaperScissorsShape::Scissors,
            (RockPaperScissorsShape::Scissors, RockPaperScissorsResult::Loss) => RockPaperScissorsShape::Paper,
        };
        self.me = Some(shape);
    }
}

pub fn solve() {
    println!("Solving Day 02!\n");

    let mut rounds = INPUT.split("\n")
        .map(|s| {
            let spl = s.split(" ").collect::<Vec<&str>>();
            let round = RockPaperScissorsRound {opponent: RockPaperScissorsShape::from_str(spl[0]).unwrap(), me: Some(RockPaperScissorsShape::from_str(spl[1]).unwrap()), result: None};
            return round;
        })
        .collect::<Vec<RockPaperScissorsRound>>();
    
    let score = rounds.iter_mut()
        .map(|round| {
            round.play_round();
            return round.result.unwrap().score() + round.me.unwrap().score();
        })
        .sum::<u64>();

    println!("Part A Solution = {}", score);

    let mut rounds_part_2 = INPUT.split("\n")
        .map(|s| {
            let spl = s.split(" ").collect::<Vec<&str>>();
            let round = RockPaperScissorsRound {opponent: RockPaperScissorsShape::from_str(spl[0]).unwrap(), me: None, result: Some(RockPaperScissorsResult::from_str(spl[1]).unwrap())};
            return round;
        })
        .collect::<Vec<RockPaperScissorsRound>>();

    let score2 = rounds_part_2.iter_mut()
        .map(|round| {
            round.determine_shape();
            return round.result.unwrap().score() + round.me.unwrap().score();
        })
        .sum::<u64>();

    println!("Part B Solution = {}", score2);
}