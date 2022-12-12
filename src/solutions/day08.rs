const INPUT: &'static str = include_str!("../../inputs/day08.txt");

fn parse_grid(input: &'static str) -> Vec<Vec<usize>> {
    let mut vec: Vec<Vec<usize>> = Vec::new();

    for line in input.split("\n") {
        let inner_vec = line.chars()
            .filter(|c| c != &'\n')
            .map(|c| {
                let s = c.to_string();
                return s.parse::<usize>().unwrap();
            })
            .collect::<Vec<usize>>();
        vec.push(inner_vec);
    }

    return vec;
}

fn is_visible(grid: &Box<Vec<Vec<usize>>>, i: usize, j: usize) -> bool {
    if i == 0 || j == 0 || i == grid.len() - 1 || j == grid[i].len() - 1 {
        // edge always visible
        return true;
    }

    let curr_height = grid[i][j];
    //println!("curr hieght {}", curr_height);
    // move "up"
    let up_res: usize = (0..i).into_iter()
        .map(|idx| grid[idx][j])
        .filter(|val| val > &curr_height)
        .collect::<Vec<usize>>()
        .len();
    if up_res == 0 {
        return true;
    }

    // move "down"
    let down_res: usize = (i+1..grid.len()).into_iter()
        .map(|idx| grid[idx][j])
        .filter(|val| val >= &curr_height)
        .collect::<Vec<usize>>()
        .len();
    if down_res == 0 {
        return true;
    }

    // move "left"
    let left_res: usize = (0..j).into_iter()
        .map(|jdx| grid[i][jdx])
        .filter(|val| val >= &curr_height)
        .collect::<Vec<usize>>()
        .len();
    if left_res == 0 {
        return true;
    }

    // move "right"
    let right_res: usize = (j+1..grid[i].len()).into_iter()
        .map(|jdx| grid[i][jdx])
        .filter(|val| val >= &curr_height)
        .collect::<Vec<usize>>()
        .len();
    if right_res == 0 {
        return true;
    }
    
    return false;
}

fn solve_part_a(input: &'static str) -> usize {

    let grid = parse_grid(input);

    /* for row in grid.clone() {
        for col in row {
            print!("{}", col);
        }
        print!("\n");
    } */

    let grid_box = Box::new(grid.to_owned());

    let max_i = grid.clone().len();
    let max_j = grid[max_i - 1].clone().len();

    //is_visible(&grid_box, 1, 1);
    let mut count: usize = 0;
    for i in 0..max_i{
        for j in 0..max_j {
            if is_visible(&grid_box, i, j) {
                if i > 0 && j > 0 && i < max_i - 1 && j < max_j - 1 {
                    println!("{} {} is visible!", i, j);   
                }
                count += 1;
            }
        }
    }

    return count;

}

pub fn solve() {
    println!("Solving Day 08!");

    let part_a = solve_part_a(INPUT);

    println!("Part A Solution = {}", part_a);
}

#[test]
pub fn test_part_a() -> Result<(), ()> {
    let input: &'static str = include_str!("../../inputs/test/day08.txt");

    assert_eq!(21, solve_part_a(input));

    Ok(())
}

#[test]
pub fn test_is_visible_1() -> Result<(), ()> {
    let input: &'static str = include_str!("../../inputs/test/day08.txt");

    let grid = parse_grid(input);

    let grid_box = Box::new(grid.to_owned());

    assert_eq!(true, is_visible(&grid_box, 1, 1));

    Ok(())
}

#[test]
pub fn test_is_visible_2() -> Result<(), ()> {
    let input: &'static str = include_str!("../../inputs/test/day08.txt");

    let grid = parse_grid(input);

    let grid_box = Box::new(grid.to_owned());

    assert_eq!(true, is_visible(&grid_box, 1, 2));

    Ok(())
}

#[test]
pub fn test_is_visible_3() -> Result<(), ()> {
    let input: &'static str = include_str!("../../inputs/test/day08.txt");

    let grid = parse_grid(input);

    let grid_box = Box::new(grid.to_owned());

    assert_eq!(false, is_visible(&grid_box, 1, 3));

    Ok(())
}

#[test]
pub fn test_is_visible_4() -> Result<(), ()> {
    let input: &'static str = include_str!("../../inputs/test/day08.txt");

    let grid = parse_grid(input);

    let grid_box = Box::new(grid.to_owned());

    assert_eq!(true, is_visible(&grid_box, 2, 1));

    Ok(())
}

#[test]
pub fn test_is_visible_5() -> Result<(), ()> {
    let input: &'static str = include_str!("../../inputs/test/day08.txt");

    let grid = parse_grid(input);

    let grid_box = Box::new(grid.to_owned());

    assert_eq!(true, is_visible(&grid_box, 2, 3));

    Ok(())
}

#[test]
pub fn test_is_visible_6() -> Result<(), ()> {
    let input: &'static str = include_str!("../../inputs/test/day08.txt");

    let grid = parse_grid(input);

    let grid_box = Box::new(grid.to_owned());

    assert_eq!(true, is_visible(&grid_box, 3, 2));

    Ok(())
}