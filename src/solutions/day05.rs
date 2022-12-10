const INPUT: &'static str = include_str!("../../inputs/day05.txt");

fn init_stacks(input: &'static str) -> Vec<Vec<&str>> {
    let mut master_vector: Vec<Vec<&str>> = Vec::new();
    for _ in 0..10 {
        master_vector.push(Vec::new());
    }

    for line in input.split('\n') {
        for (stack_index, chunk) in line.split("").array_chunks::<4>().enumerate() {
            //println!("{:?}", chunk);
            // relevant char will always be at post 2
            if chunk[2] != " " {
                master_vector[stack_index].push(chunk[2]);
            }
        }
    }

    master_vector = master_vector.iter()
        .map(|v| v.iter().copied().rev().collect())
        .collect::<Vec<Vec<&str>>>();

    return master_vector;
}

fn solve_part_a(input: &'static str) -> String {
    let mut splt = input.split("==========");

    let mut stacks = init_stacks(&splt.next().unwrap());

    let instructions = &splt.next().unwrap();

    for line in instructions.split("\n").skip(1) {
        //println!("{}", line);
        let mut splt = line.split(" ");
        let _ = splt.next().unwrap();
        let mut amount = splt.next().unwrap().parse::<u32>().unwrap();
        let _ = splt.next().unwrap();
        let from = splt.next().unwrap().parse::<usize>().unwrap() - 1;
        let _ = splt.next().unwrap();
        let to = splt.next().unwrap().parse::<usize>().unwrap() - 1;

        while amount > 0 {
            let item = stacks[from].pop().unwrap();
            stacks[to].push(item);
            amount = amount - 1;
        }
    }

    let mut res = "".to_owned();
    
    for mut stack in stacks {
        let item = stack.pop();
        match item {
            Some(i) => res.push_str(i),
            None => break
        };
    }

    return res;
}

fn solve_part_b(input: &'static str) -> String {
    let mut splt = input.split("==========");

    let mut stacks = init_stacks(&splt.next().unwrap());

    let instructions = &splt.next().unwrap();

    for line in instructions.split("\n").skip(1) {
        //println!("{}", line);
        let mut splt = line.split(" ");
        let _ = splt.next().unwrap();
        let mut amount = splt.next().unwrap().parse::<u32>().unwrap();
        let _ = splt.next().unwrap();
        let from = splt.next().unwrap().parse::<usize>().unwrap() - 1;
        let _ = splt.next().unwrap();
        let to = splt.next().unwrap().parse::<usize>().unwrap() - 1;

        let mut temp_stack: Vec<&str> = Vec::new();

        while amount > 0 {
            let item = stacks[from].pop().unwrap();
            temp_stack.push(item);
            amount = amount - 1;
        }

        while temp_stack.len() > 0 {
            stacks[to].push(temp_stack.pop().unwrap());
        }
    }

    let mut res = "".to_owned();
    
    for mut stack in stacks {
        let item = stack.pop();
        match item {
            Some(i) => res.push_str(i),
            None => break
        };
    }

    return res;
}

pub fn solve() {
    println!("Solving Day 5!");

    let part_a = solve_part_a(INPUT);

    println!("Part A solution = {}", part_a);

    let part_b = solve_part_b(INPUT);

    println!("Part B solution = {}", part_b);

    /* for stack in stacks {
        println!("{:?}", stack);
    } */
}

#[test]
fn test_part_a() -> Result<(), ()> {
    let input: &'static str = include_str!("../../inputs/test/day05.txt");

    let res = solve_part_a(input);

    assert_eq!("CMZ", res);

    Ok(())
}

#[test]
fn test_part_b() -> Result<(), ()> {
    let input: &'static str = include_str!("../../inputs/test/day05.txt");

    let res = solve_part_b(input);

    assert_eq!("MCD", res);

    Ok(())
}