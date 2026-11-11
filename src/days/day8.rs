pub fn part1() {
    let input = std::fs::read_to_string("data/day8.txt").unwrap();

    let answer = process_input(&input);

    println!("{}", answer);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day8.txt").unwrap();

    let answer = process_input_2(&input);

    println!("{}", answer);
}

fn process_input(input: &str) -> u64 {
    let mut input = input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .into_iter();

    let mut checksum = 0;

    process_tree(&mut input, &mut checksum);

    checksum
}

fn process_tree(input: &mut impl Iterator<Item = u64>, checksum: &mut u64) {
    let node_count = input.next().unwrap();
    let metadata_count = input.next().unwrap();

    for _ in 0..node_count {
        process_tree(input, checksum);
    }

    for _ in 0..metadata_count {
        *checksum += input.next().unwrap();
    }
}

fn process_input_2(input: &str) -> u64 {
    let mut input = input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .into_iter();

    process_tree_2(&mut input)
}

fn process_tree_2(input: &mut impl Iterator<Item = u64>) -> u64 {
    let node_count = input.next().unwrap();
    let metadata_count = input.next().unwrap();

    let mut child_values = Vec::new();

    for _ in 0..node_count {
        child_values.push(process_tree_2(input));
    }

    let mut node_value = 0;

    for _ in 0..metadata_count {
        let metadata_value = input.next().unwrap();

        if node_count == 0 {
            node_value += metadata_value;
        } else {
            let metadata_value = metadata_value as usize - 1;

            if metadata_value < child_values.len() {
                node_value += child_values[metadata_value];
            }
        }
    }

    node_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

        assert_eq!(138, process_input(input));
    }

    #[test]
    fn test2() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

        assert_eq!(66, process_input_2(input));
    }
}
