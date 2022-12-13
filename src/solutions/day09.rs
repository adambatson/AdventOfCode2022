use std::{collections::HashSet, str::FromStr};

const INPUT: &'static str = include_str!("../../inputs/day09.txt");

enum Command {
    Right(usize),
    Left(usize),
    Up(usize),
    Down(usize)
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, dist) = s.split_once(" ").unwrap();
        let dist_num: usize = dist.parse::<usize>().unwrap();
        return match direction {
            "R" => Ok(Command::Right(dist_num)),
            "L" => Ok(Command::Left(dist_num)),
            "U" => Ok(Command::Up(dist_num)),
            "D" =>Ok(Command::Down(dist_num)),
            _    => Err(())
        };
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize
}

struct Knot {
    curr_point: Point,
    visited_points: HashSet<Point>
}

impl Knot {
    fn follow(&mut self, other: Point) {
        let diff_x = other.x as isize- self.curr_point.x as isize;
        let diff_y = other.y as isize - self.curr_point.y as isize;

        if diff_x.abs() == 1 && diff_y.abs() == 1 {
            return;
        }

        if diff_x.abs() > 1 && diff_y.abs() > 1 {
            if diff_x > 0 {
                self.curr_point.x += 1;
            } else {
                self.curr_point.x -= 1;
            }
            if diff_y > 0 {
                self.curr_point.y += 1;
            } else {
                self.curr_point.y -= 1;
            }
        } else if diff_x.abs() > 1 {

            if diff_x > 0 {
                self.curr_point.x += 1;
            } else {
                self.curr_point.x -= 1;
            }
            self.curr_point.y = other.y;
        } else if diff_y.abs() > 1 {

            if diff_y > 0 {
                self.curr_point.y += 1;
            } else {
                self.curr_point.y -= 1;
            }
            self.curr_point.x = other.x;
        };
        self.visited_points.insert(self.curr_point);
    }
}

struct Board {
    knots: Vec<Knot>
}

impl Board {
    fn new() -> Self {
        let mut knots = Vec::new();
        for _ in 0..11 {
            let p = Point {x: 500, y: 500};
            knots.push(Knot {curr_point: p, visited_points: HashSet::new()});
        }

        let board = Board {
            knots: knots
        };
        return board;
    }

    fn handle_move(&mut self, line: &'static str) {
        let cmd = Command::from_str(line).unwrap();


        match cmd {
            Command::Up(dist) => {
                for _ in 0..dist {
                    self.move_up()
                }
            },
            Command::Down(dist) => {
                for _ in 0..dist {
                    self.move_down()
                }
            },
            Command::Left(dist) => {
                for _ in 0..dist {
                    self.move_left()
                }
            },
            Command::Right(dist) => {
                for _ in 0..dist {
                    self.move_right()
                }
            },
        }
    }

    fn move_right(&mut self) {
        self.knots[0].curr_point.x += 1;
        self.move_knots();
    }

    fn move_left(&mut self) {
        self.knots[0].curr_point.x -= 1;
        self.move_knots();
    }

    fn move_down(&mut self) {
        self.knots[0].curr_point.y -= 1;
        self.move_knots();
    }

    fn move_up(&mut self) {
        self.knots[0].curr_point.y += 1;
        self.move_knots();
    }

    fn move_knots(&mut self) {
        for i in 1..self.knots.len() {
            let other_point = self.knots[i - 1].curr_point;
            self.knots[i].follow(other_point);
        }
    }
}

pub fn solve() {
    println!("Solving Day 9!");

    let mut board = Board::new();

    for line in INPUT.split("\n") {
        board.handle_move(line);
    }

    let part_a = board.knots[1].visited_points.len();

    println!("Part A Solution = {}", part_a);

    let part_b = board.knots[9].visited_points.len();

    println!("Part Board Solution = {}", part_b);
}

#[test]
pub fn test_part_a() -> Result<(), ()> {
    let input: &'static str = include_str!("../../inputs/test/day09.txt");

    let mut board = Board::new();

    for line in input.split("\n") {
        board.handle_move(line);
    }

    let part_a = board.knots[1].visited_points.len();

    assert_eq!(13, part_a);

    Ok(())
}

#[test]
pub fn test_actual_part_a() -> Result<(), ()> {
    let mut board = Board::new();

    for line in INPUT.split("\n") {
        board.handle_move(line);
    }

    let part_a = board.knots[1].visited_points.len();

    assert_eq!(6018, part_a);

    Ok(())
}