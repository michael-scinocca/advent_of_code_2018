use std::collections::HashMap;

pub fn part1() {
    let input = std::fs::read_to_string("data/day2.txt").unwrap();

    let mut two_count = 0;
    let mut three_count = 0;

    for line in input.trim().lines() {
        let mut char_map = HashMap::new();

        for char in line.chars() {
            char_map.entry(char).and_modify(|v| *v += 1).or_insert(1);
        }

        if char_map.values().any(|v| *v == 2) {
            two_count += 1;
        }

        if char_map.values().any(|v| *v == 3) {
            three_count += 1;
        }
    }

    println!("{}", two_count * three_count);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day2.txt").unwrap();

    let lines = input.trim().lines().collect::<Vec<_>>();

    let mut solution = String::new();

    'outer: for i in 0..(lines.len() - 1) {
        for j in (i + 1)..lines.len() {
            let test = lines[i]
                .chars()
                .zip(lines[j].chars())
                .filter_map(|e| if e.0 == e.1 { Some(e.0) } else { None })
                .collect::<String>();

            if test.len() == lines[i].len() - 1 {
                solution = test;

                break 'outer;
            }
        }
    }

    println!("{}", solution);
}
