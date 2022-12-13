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

struct Board {
    cells: Vec<Vec<char>>,
    head: Point,
    tail: Point,
    tail_points: HashSet<Point>
}

impl Board {
    fn new() -> Self {
        let mut grid = vec![vec!['.'; 1000]; 1000];
        grid[500][500] = 'H';
        let mut board = Board {
            cells: grid,
            head: Point {x: 500, y: 500},
            tail: Point {x: 500, y: 500},
            tail_points: HashSet::new()
        };
        board.tail_points.insert(board.tail);
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
        self.head.x += 1;
        self.move_tail();
    }

    fn move_left(&mut self) {
        self.head.x -= 1;
        self.move_tail();
    }

    fn move_down(&mut self) {
        self.head.y -= 1;
        self.move_tail();
    }

    fn move_up(&mut self) {
        self.head.y += 1;
        self.move_tail();
    }

    fn move_tail(&mut self) {
        let diff_x = self.head.x as isize- self.tail.x as isize;
        let diff_y = self.head.y as isize - self.tail.y as isize;

        if diff_x.abs() == diff_y.abs() {
            return;
        }

        if diff_x.abs() > 1 {
            if diff_x > 0 {
                self.tail.x += 1;
            } else {
                self.tail.x -= 1;
            }
            self.tail.y = self.head.y;
        }
        if diff_y.abs() > 1 {
            if diff_y > 0 {
                self.tail.y += 1;
            } else {
                self.tail.y -= 1;
            }
            self.tail.x = self.head.x;
        }
        self.tail_points.insert(self.tail);
    }
}

pub fn solve() {
    println!("Solving Day 9!");

    let mut board = Board::new();

    for line in INPUT.split("\n") {
        board.handle_move(line);
    }

    let part_a = board.tail_points.len();

    println!("Part A Solution = {}", part_a);
}

#[test]
pub fn test_part_a() -> Result<(), ()> {
    let input: &'static str = include_str!("../../inputs/test/day09.txt");

    let mut board = Board::new();

    for line in INPUT.split("\n") {
        board.handle_move(line);
    }

    let part_a = board.tail_points.len();

    assert_eq!(13, part_a);

    Ok(())
}