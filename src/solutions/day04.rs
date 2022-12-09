use std::{str::FromStr, num::ParseIntError, string::ParseError, convert::Infallible};

const INPUT: &'static str = include_str!("../../inputs/day04.txt");

struct Section {
    start: usize,
    end: usize
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

pub fn solve() {

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
pub fn test_setion_from_str_error() -> Result<(), ()> {
    let str = "4-5-6";

    let section = Section::from_str(str);

    //assert_eq!(Err(ParseError), section);

    Ok(())
}