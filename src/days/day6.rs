use std::collections::HashMap;

pub fn part1() {
    let input = std::fs::read_to_string("data/day6.txt").unwrap();

    let answer = process_input(&input, 500);

    println!("{}", answer);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day6.txt").unwrap();

    let answer = process_input_2(&input, 500, 10000);

    println!("{}", answer);
}

fn process_input(input: &str, size: i64) -> u64 {
    let mut coord_map: HashMap<(i64, i64), u64> = HashMap::new();

    for line in input
        .trim()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
    {
        let coordinate = {
            let mut parts = line.split(",");

            (
                parts.next().unwrap().trim().parse::<i64>().unwrap(),
                parts.next().unwrap().trim().parse::<i64>().unwrap(),
            )
        };

        coord_map.insert(coordinate, 0);
    }

    let mut check_coord_map = coord_map.clone();

    run_areas(&mut coord_map, size);
    run_areas(&mut check_coord_map, size * 2);

    coord_map
        .iter()
        .map(|(k, v)| {
            (
                k,
                if check_coord_map.get(k).unwrap() == v {
                    *v
                } else {
                    0
                },
            )
        })
        .map(|c| c.1)
        .max()
        .unwrap()
}

fn run_areas(coord_map: &mut HashMap<(i64, i64), u64>, size: i64) {
    for x in -size..=size {
        for y in -size..=size {
            let mut closest_distance = i64::MAX;
            let mut closest_coord = None;
            let mut closest_tie = false;

            for coord in coord_map.keys() {
                let distance = (x - coord.0).abs() + (y - coord.1).abs();

                if distance < closest_distance {
                    closest_distance = distance;
                    closest_coord = Some(coord);
                    closest_tie = false;
                } else if distance == closest_distance {
                    closest_tie = true;
                }
            }

            if closest_tie {
                continue;
            }

            if let Some(&coord) = closest_coord
                && let Some(val) = coord_map.get_mut(&coord)
            {
                *val += 1;
            }
        }
    }
}

fn process_input_2(input: &str, size: i64, max_distance: i64) -> u64 {
    let mut coord_map = Vec::new();

    for line in input
        .trim()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
    {
        let coordinate = {
            let mut parts = line.split(",");

            (
                parts.next().unwrap().trim().parse::<i64>().unwrap(),
                parts.next().unwrap().trim().parse::<i64>().unwrap(),
            )
        };

        coord_map.push(coordinate);
    }

    let mut region_size = 0;

    for x in -size..=size {
        for y in -size..=size {
            let mut distance_sum = 0;

            for coord in &coord_map {
                distance_sum += (x - coord.0).abs() + (y - coord.1).abs();
            }

            if distance_sum >= max_distance {
                continue;
            }

            region_size += 1;
        }
    }

    region_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = r"
            1, 1
            1, 6
            8, 3
            3, 4
            5, 5
            8, 9
        ";

        assert_eq!(17, process_input(&input, 20));
    }

    #[test]
    fn test2() {
        let input = r"
            1, 1
            1, 6
            8, 3
            3, 4
            5, 5
            8, 9
        ";

        assert_eq!(16, process_input_2(&input, 20, 32));
    }
}
