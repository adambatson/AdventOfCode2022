#![feature(iter_array_chunks)]

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use std::fmt::{self};
use std::io::{self};
use std::str::FromStr;


mod solutions {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
}

#[derive(Debug, PartialEq, EnumIter)]
enum Solution {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7
}

impl FromStr for Solution {
    type Err = ();

    fn from_str(input: &str) -> Result<Solution, Self::Err> {
        match input {
            "1"  => Ok(Solution::Day1),
            "2"  => Ok(Solution::Day2),
            "3"  => Ok(Solution::Day3),
            "4"  => Ok(Solution::Day4),
            "5"  => Ok(Solution::Day5),
            "6"  => Ok(Solution::Day6),
            "7"  => Ok(Solution::Day7),
            _    => Err(())
        }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Solution::Day1 => write!(f, "Day 1 Calorie Counting"),
            Solution::Day2 => write!(f, "Day 2: Rock Paper Scissors"),
            Solution::Day3 => write!(f, "Day 3: Rucksack Reorganization"),
            Solution::Day4 => write!(f, "Day 4: Camp Cleanup"),
            Solution::Day5 => write!(f, "Day 5: Supply Stacks"),
            Solution::Day6 => write!(f, "Day 6: Tuning Trouble"),
            Solution::Day7 => write!(f, "Day 7: No Space Left On Device")
        }
    }
}

fn get_user_input() -> String {
    let mut str = String::new();
    match io::stdin().read_line(&mut str) {
        Ok(_) => {
            let trimmed_input = str.trim();
            return trimmed_input.to_string();
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

fn get_solution(str:String) -> Solution{
    return Solution::from_str(&str).unwrap();
}

fn main_loop() {
    loop {

        for sol in Solution::iter() {
            println!("{}", sol)
        }

        let inpt = get_user_input();

        if inpt == "exit" {
            return;
        }

        let sol = get_solution(inpt);

        match sol {
            Solution::Day1 => solutions::day01::solve(),
            Solution::Day2 => solutions::day02::solve(),
            Solution::Day3 => solutions::day03::solve(),
            Solution::Day4 => solutions::day04::solve(),
            Solution::Day5 => solutions::day05::solve(),
            Solution::Day6 => solutions::day06::solve(),
            Solution::Day7 => solutions::day07::solve()
        }
    }
}

fn main() {
    println!("Hello, world!");

    main_loop();
}
