
const INPUT: &'static str = include_str!("../../inputs/day01.txt");

pub fn solve() {
    println!("Solving Day 01!\n");

    let calories_vec: Vec<Vec<u32>> = INPUT
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .map(|num| {
                    num.parse::<u32>()
                        .unwrap_or_else(|v| panic!("Num is invalid: {}", v))
                })
                .collect()
        })
        .collect();

    let mut sum_calories : Vec<u32> = calories_vec.iter().map(|c| c.iter().sum()).collect();

    sum_calories.sort_by(|a, b| b.cmp(a));

    println!("Part A Solution {}", sum_calories.first().unwrap());

    let mut top_three : u32 = 0;

    for i in 0..3 {
        top_three += sum_calories[i];
    }

    println!("\nPart B Solution {}", top_three);

}