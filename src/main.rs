use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use std::fmt::{self};
use std::io::{self};
use std::str::FromStr;


mod solutions {
    pub mod day01;
}

#[derive(Debug, PartialEq, EnumIter)]
enum Solution {
    Day1
}

impl FromStr for Solution {
    type Err = ();

    fn from_str(input: &str) -> Result<Solution, Self::Err> {
        match input {
            "1"  => Ok(Solution::Day1),
            _    => Err(())
        }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Solution::Day1 => write!(f, "Day 1 Calorie Counting")
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
            Solution::Day1 => solutions::day01::solve()
        }
    }
}

fn main() {
    println!("Hello, world!");

    main_loop();
}
