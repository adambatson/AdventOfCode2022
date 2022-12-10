use std::{str::FromStr};

const INPUT: &'static str = include_str!("../../inputs/day04.txt");

#[derive(Debug, Clone)]
struct Section {
    start: usize,
    end: usize
}

impl Section {
    fn fully_contains(&self, other: Section) -> bool {
        return (other.start >= self.start && other.end <= self.end) || (self.start >= other.start && self.end <= other.end);
    }

    fn overlaps(&self, other: Section) -> bool {
        return
        // self includes other start
            (other.start >= self.start && other.start <= self.end) ||
        // self includes other end
            (other.end >= self.start && other.end <= self.end) ||
        // other includes self start
            (self.start >= other.start && self.start <= other.end) ||
        // other includes self end
            (self.end >= other.start && self.end <= other.end)
    }
}

impl FromStr for Section {

    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {

        let mut splt = input.split('-');

        if splt.clone().count() != 2 {
            return Err(());
        }

        //let iter = splt.into_iter();
        let start: usize = splt.next().unwrap().parse::<usize>().expect("Could not parse start");
        let end: usize = splt.next().unwrap().parse::<usize>().expect("Could not parse end");

        Ok(Section {start: start, end: end})
    }
}

fn solve_part_a(input: &'static str) -> usize {

    let lines = input.split('\n')
        .map(|l| {
            let mut splt = l.split(',');
            let s1 = splt.next().unwrap().parse::<Section>().unwrap();
            let s2 = splt.next().unwrap().parse::<Section>().unwrap();
            return (s1, s2)
        })
        .collect::<Vec<(Section, Section)>>();

    let res = lines.iter()
        .filter(|tuple| {
            tuple.0.fully_contains(tuple.1.to_owned())
        })
        .count();

    return res;
}

fn solve_part_b(input: &'static str) -> usize {

    let lines = input.split('\n')
        .map(|l| {
            let mut splt = l.split(',');
            let s1 = splt.next().unwrap().parse::<Section>().unwrap();
            let s2 = splt.next().unwrap().parse::<Section>().unwrap();
            return (s1, s2)
        })
        .collect::<Vec<(Section, Section)>>();

    let res = lines.iter()
        .filter(|tuple| {
            let contains = tuple.0.overlaps(tuple.1.to_owned());
            println!("{:?} {:?} {:?}", tuple.0, tuple.1, contains);
            return contains;
        })
        .count();

    return res;
}

pub fn solve() {
    println!("Solving Day 4!");

    let part_a = solve_part_a(INPUT);

    println!("Part A Solution = {}", part_a);

    let part_b = solve_part_b(INPUT);

    println!("Part B Solution = {}", part_b);
}

#[test]
pub fn test_section_from_str() -> Result<(), ()> {

    let str = "2-6";

    let section = Section::from_str(str).unwrap();

    assert_eq!(2, section.start);
    assert_eq!(6, section.end);

    Ok(())
}

#[test]
#[should_panic]
pub fn test_setion_from_str_error(){
    let str = "4-5-6";

    Section::from_str(str).unwrap();
}

#[test]
#[should_panic]
pub fn test_setion_from_str_error_not_numbers(){
    let str = "z-a";

    Section::from_str(str).unwrap();
}

#[test]
fn test_section_full_contains() {
    let sec1 = Section{start: 2, end: 8};
    let sec2 = Section{start: 3, end: 7};

    assert!(sec1.fully_contains(sec2));
}

#[test]
fn test_solve_a() -> Result<(), ()> {
    let input: &'static str = include_str!("../../inputs/test/day04.txt");

    let res = solve_part_a(input);

    assert_eq!(2, res);

    Ok(())
}

#[test]
fn test_solve_b() -> Result<(), ()> {
    let input: &'static str = include_str!("../../inputs/test/day04.txt");

    let res = solve_part_b(input);

    assert_eq!(4, res);

    Ok(())
}