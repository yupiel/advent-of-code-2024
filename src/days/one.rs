use std::fs::read_to_string;

pub fn run() -> Result<(), ()> {
    let input_lines: Vec<String> = read_to_string("src/inputs/day_1")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut left_side = Vec::<i32>::new();
    let mut right_side = Vec::<i32>::new();

    for line in input_lines {
        let numberinos: Vec<&str> = line.split("   ").collect();

        left_side.push(numberinos[0].parse().unwrap());
        right_side.push(numberinos[1].parse().unwrap());
    }

    left_side.sort();
    right_side.sort();

    let mut distances = Vec::<i32>::new();
    let mut similarity = Vec::<i32>::new();

    for (left, right) in left_side.iter().zip(right_side.iter()) {
        distances.push(left.abs_diff(*right) as i32);

        similarity.push(left * right_side.iter().filter(|x| *x == left).count() as i32);
    }

    println!("Day one - Part one: {}", distances.iter().sum::<i32>());
    println!("Day one - Part two: {}", similarity.iter().sum::<i32>());

    Ok(())
}
