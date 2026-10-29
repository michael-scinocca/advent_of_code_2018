use std::collections::{HashMap, HashSet};

pub fn part1() {
    let input = std::fs::read_to_string("data/day3.txt").unwrap();

    let fabric = get_fabric(&input);

    let multi_claims = fabric.values().filter(|c| c.len() > 1).count();

    println!("{}", multi_claims);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day3.txt").unwrap();

    let fabric = get_fabric(&input);

    let single_claims = fabric
        .values()
        .filter(|c| c.len() == 1)
        .map(|c| c.first().unwrap())
        .collect::<HashSet<_>>();

    let multi_claims = fabric.values().filter(|c| c.len() > 1).collect::<Vec<_>>();

    for single_claim in &single_claims {
        let count = multi_claims
            .iter()
            .filter(|v| v.contains(single_claim))
            .count();

        if count == 0 {
            println!("{}", single_claim);
            break;
        }
    }
}

fn get_fabric(input: &str) -> HashMap<(u32, u32), Vec<&str>> {
    let mut fabric: HashMap<(u32, u32), Vec<&str>> = HashMap::new();

    for line in input.trim().lines().map(|l| l.trim()) {
        let mut parts = line.split("@");

        let id = parts.next().unwrap().trim().trim_start_matches("#");

        parts = parts.next().unwrap().trim().split(":");

        let mut coords = parts.next().unwrap().trim().split(",");
        let x_start = coords.next().unwrap().parse::<u32>().unwrap();
        let y_start = coords.next().unwrap().parse::<u32>().unwrap();

        let mut size = parts.next().unwrap().trim().split("x");
        let width = size.next().unwrap().parse::<u32>().unwrap();
        let height = size.next().unwrap().parse::<u32>().unwrap();

        for x in x_start..(x_start + width) {
            for y in y_start..(y_start + height) {
                let key = (x, y);

                fabric
                    .entry(key)
                    .and_modify(|claims| claims.push(id))
                    .or_insert_with(|| vec![id]);
            }
        }
    }

    fabric
}
