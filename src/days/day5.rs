pub fn part1() {
    let input = std::fs::read_to_string("data/day5.txt").unwrap();

    let answer = process_input(&input);

    println!("{}", answer.len());
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day5.txt").unwrap();

    let mut lowest = usize::MAX;

    for c in 'A'..='Z' {
        let test_input = input.replace([c, c.to_ascii_lowercase()], "");

        let answer = process_input(&test_input);

        if answer.len() < lowest {
            lowest = answer.len();
        }
    }

    println!("{}", lowest);
}

fn process_input(input: &str) -> String {
    let mut input = input.to_string();

    loop {
        let mut next = String::new();

        let mut windows = input.as_bytes().windows(2).peekable();

        while let Some(window) = windows.next() {
            if window[0] != window[1] && window[0].eq_ignore_ascii_case(&window[1]) {
                windows.next();
                continue;
            }

            next.push(window[0] as char);

            if windows.peek().is_none() {
                next.push(window[1] as char);
            }
        }

        if input.len() == next.len() {
            break next;
        }

        input = next;
    }
}
