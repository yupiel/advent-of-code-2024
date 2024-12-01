use std::fs::read_to_string;

pub fn run() -> Result<(), ()> {
    let (mut old_location_ids, mut new_location_ids) = read_to_string("src/inputs/day_1")
        .unwrap()
        .lines()
        .map(|line| {
            let values: Vec<&str> = line.split("   ").collect();
            (
                values[0].parse::<i32>().unwrap(),
                values[1].parse::<i32>().unwrap(),
            )
        })
        .collect::<(Vec<i32>, Vec<i32>)>();

    old_location_ids.sort();
    new_location_ids.sort();

    let mut distances = Vec::<i32>::new();
    let mut similarity = Vec::<i32>::new();

    for (left, right) in old_location_ids.iter().zip(new_location_ids.iter()) {
        distances.push(left.abs_diff(*right) as i32);
        similarity.push(left * new_location_ids.iter().filter(|x| *x == left).count() as i32);
    }

    println!("Day one - Part one: {}", distances.iter().sum::<i32>());
    println!("Day one - Part two: {}", similarity.iter().sum::<i32>());

    Ok(())
}
