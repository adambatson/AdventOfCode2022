use std::str::{FromStr, Split};

const INPUT: &'static str = include_str!("../../inputs/day10.txt");

enum Command {
    Add(isize),
    Noop
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s == "noop" {
            return Ok(Self::Noop);
        }
        
        let (_, amnt) = s.split_once(" ").unwrap();

        println!("{}", amnt);

        let amount = amnt.parse::<isize>().unwrap();

        return Ok(Self::Add(amount));
    }
}

struct Processor<> {
    wait: isize,
    ip: usize,
    curr_cycle: isize,
    reg_x: isize,
    instructions: Vec<String>,
    reg_x_history: Vec<isize>
}

impl Processor {
    fn new(instructions: Vec<String>) -> Processor {
        let mut p =  Processor { 
            wait: 0,
            ip: 0,
            curr_cycle: 0,
            reg_x: 1,
            instructions: instructions,
            reg_x_history: Vec::new()
        };
        p.reg_x_history.push(1);
        return p;
    }

    fn tick(&mut self) {
        // track val at start of tick
        self.reg_x_history.push(self.reg_x);

        // execute next instruction
        match Command::from_str(&self.instructions[self.ip]).unwrap() {
            Command::Noop => {
                self.ip += 1;
            },
            Command::Add(amount) => {
                // takes two cycles
                self.reg_x_history.push(self.reg_x);
                self.reg_x += amount;
                self.ip += 2;
            }
        }
    }

    fn run_program(&mut self) {
        while self.ip < self.instructions.len() {
            self.tick();
        }
    }
}

fn solve_part_a(input: &'static str) -> isize {
    let instructions = input.split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();


    let mut processor = Processor::new(instructions);
    processor.run_program();

    return processor.reg_x_history[19]
        + processor.reg_x_history[59]
        + processor.reg_x_history[99]
        + processor.reg_x_history[139]
        + processor.reg_x_history[179]
        + processor.reg_x_history[219]
}

pub fn solve() {
    println!("Solving Day 10!");

    let part_a = solve_part_a(INPUT);

    println!("Part A Solution = {}", part_a);
}