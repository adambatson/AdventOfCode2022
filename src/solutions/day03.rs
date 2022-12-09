use std::{collections::{HashMap, HashSet}};

const INPUT: &'static str = include_str!("../../inputs/day03.txt");

const LOWER: &'static str = "abcdefghijklmnopqrstuvwxyz";
const UPPER: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";    

fn build_letter_map() -> HashMap<String, usize> {
    let mut letter_map: HashMap<String, usize> = HashMap::new();

    let mut i: usize = 1;
    for l in LOWER.split("").filter(|&s| s != "") {
        letter_map.insert(l.to_string(), i);
        i += 1
    }

    for l in UPPER.split("").filter(|&s| s != "") {
        letter_map.insert(l.to_string(), i);
        i += 1;
    }

    return letter_map;
}

fn split_rucksack(rucksack: &str) -> (Vec<&str>, Vec<&str>) {
    let splt = rucksack.split("").filter(|&s| s != "").collect::<Vec<&str>>();
    let (first, second) = splt.split_at(splt.len() / 2);
    return (first.to_vec(), second.to_vec());
}

fn get_rucksack_intersection(input: (Vec<&str>, Vec<&str>)) -> String {
    let first: HashSet<&str> = HashSet::from_iter(input.0);
    let second: HashSet<&str> = HashSet::from_iter(input.1);

    let mut intersect = first.intersection(&second);
    return intersect.next().unwrap().clone().to_string();
}

fn solve_part_a(input: &'static str) -> usize {
    let letter_map = build_letter_map();

    println!("{:?}", letter_map);

    let part_a: usize = input.split("\n")
        .map(|line| {
           let splt = split_rucksack(line) ;
           let intersect = get_rucksack_intersection(splt);
           return letter_map[&intersect];
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    return part_a;
}

pub fn solve() {

    println!("Solving Day 03\n");

    let part_a = solve_part_a(INPUT);

    println!("Part A Solution = {}", part_a);
}

#[test]
fn test_split_rucksack() -> Result<(), ()> {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
    let expec_first = "vJrwpWtwJgWr".split("").filter(|&s| s != "").collect::<Vec<&str>>();
    let expec_second = "hcsFMMfFFhFp".split("").filter(|&s| s != "").collect::<Vec<&str>>();

    let res = split_rucksack(input);

    assert_eq!(expec_first, res.0);
    assert_eq!(expec_second, res.1);

    Ok(())
}

#[test]
fn test_get_rucksack_intersection() -> Result<(), ()> {
    let first = "vJrwpWtwJgWr".split("").filter(|&s| s != "").collect::<Vec<&str>>();
    let second = "hcsFMMfFFhFp".split("").filter(|&s| s != "").collect::<Vec<&str>>();

    let res = get_rucksack_intersection((first, second));

    assert_eq!("p", res);

    Ok(())
}

#[test]
fn test_solve_a() -> Result<(), ()> {
    let input: &'static str = include_str!("../../inputs/test/day03.txt");

    let res = solve_part_a(input);

    assert_eq!(157, res);

    Ok(())
}