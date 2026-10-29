use std::collections::HashSet;

pub fn part1() {
    let input = std::fs::read_to_string("data/day1.txt").unwrap();

    let mut frequency = 0;

    for line in input.trim().lines() {
        frequency += line.parse::<i32>().unwrap();
    }

    println!("{}", frequency);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day1.txt").unwrap();

    let mut frequencies = HashSet::new();

    let mut frequency = 0;

    frequencies.insert(frequency);

    'outer: loop {
        for line in input.trim().lines() {
            frequency += line.parse::<i32>().unwrap();

            if !frequencies.insert(frequency) {
                break 'outer;
            }
        }
    }

    println!("{}", frequency);
}
