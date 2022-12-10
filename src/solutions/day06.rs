use std::collections::HashSet;

const INPUT: &'static str = include_str!("../../inputs/day06.txt");

fn find_first_unique_window(input: &'static str, window_size: usize) -> usize {
    let char_vec : Vec<char> = input.chars().collect();
    for (index, window) in char_vec.windows(window_size).enumerate() {
        //println!("{:?}", window);
        let mut set: HashSet<&char> = HashSet::new();
        for c in window {
            if set.contains(c) {
                break;
            }
            set.insert(c);
        }
        if set.len() == window_size {
            return index + window_size;
        }
    }

    return 0;
}

pub fn solve() {
    println!("Solving Day 6!");

    let part_a = find_first_unique_window(INPUT, 4);

    println!("Part A solution = {}", part_a);

    let part_b = find_first_unique_window(INPUT, 14);

    println!("Part B solution = {}", part_b);
}

#[test]
fn test_part_a_example_a() -> Result<(), ()> {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    assert_eq!(7, find_first_unique_window(input, 4));

    Ok(())
}

#[test]
fn test_part_a_example_b() -> Result<(), ()> {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";

    assert_eq!(5, find_first_unique_window(input, 4));

    Ok(())
}