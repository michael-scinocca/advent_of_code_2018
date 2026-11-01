use std::collections::HashMap;

pub fn part1() {
    let input = std::fs::read_to_string("data/day4.txt").unwrap();

    let guard_sleep_schedule = process_input(&input);

    let guard = find_guard(&guard_sleep_schedule);

    println!("{}", guard);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day4.txt").unwrap();

    let guard_sleep_schedule = process_input(&input);

    let guard = find_guard_2(&guard_sleep_schedule);

    println!("{}", guard);
}

fn process_input(input: &str) -> HashMap<u32, Vec<(u32, u32, u32)>> {
    let mut guard_sleep_schedule: HashMap<u32, Vec<(u32, u32, u32)>> = HashMap::new();

    let mut current_guard = None;
    let mut current_sleep_time = None;

    let mut input = input
        .trim()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    input.sort();

    for line in input {
        let mut parts = line.split_whitespace();

        let mut time_parts = parts.nth(1).unwrap().trim_end_matches("]").split(":");

        let _hour = time_parts.next().unwrap().parse::<u32>().unwrap();
        let minute = time_parts.next().unwrap().parse::<u32>().unwrap();

        match parts.next().unwrap() {
            "Guard" => {
                current_guard = Some(
                    parts
                        .next()
                        .unwrap()
                        .trim_start_matches("#")
                        .parse()
                        .unwrap(),
                );
            }
            "falls" => {
                current_sleep_time = Some(minute);
            }
            "wakes" => {
                guard_sleep_schedule
                    .entry(current_guard.unwrap())
                    .and_modify(|s| {
                        s.push((
                            current_sleep_time.unwrap(),
                            minute,
                            minute - current_sleep_time.unwrap(),
                        ))
                    })
                    .or_insert_with(|| {
                        vec![(
                            current_sleep_time.unwrap(),
                            minute,
                            minute - current_sleep_time.unwrap(),
                        )]
                    });
            }
            _ => panic!("Unknown line {}", line),
        }
    }

    guard_sleep_schedule
}

fn find_guard(guard_sleep_schedule: &HashMap<u32, Vec<(u32, u32, u32)>>) -> u32 {
    let guard = guard_sleep_schedule
        .iter()
        .map(|(g, s)| (g, s.iter().map(|s| s.2).sum::<u32>()))
        .max_by_key(|g| g.1)
        .unwrap();

    let mut sleep_map = HashMap::new();

    for sleep in &guard_sleep_schedule[guard.0] {
        for minute in sleep.0..sleep.1 {
            sleep_map.entry(minute).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    let minute = sleep_map.iter().max_by_key(|s| s.1).unwrap().0;

    guard.0 * minute
}

fn find_guard_2(guard_sleep_schedule: &HashMap<u32, Vec<(u32, u32, u32)>>) -> u32 {
    let guard = guard_sleep_schedule
        .iter()
        .map(|(g, s)| {
            let mut sleep_map = HashMap::new();

            for sleep in s {
                for minute in sleep.0..sleep.1 {
                    sleep_map.entry(minute).and_modify(|v| *v += 1).or_insert(1);
                }
            }

            let (minute, count) = sleep_map.iter().max_by_key(|s| s.1).unwrap();

            (g, *minute, *count)
        })
        .max_by_key(|g| g.2)
        .unwrap();

    guard.0 * guard.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let guard_sleep_schedule = process_input(
            r"
            [1518-11-01 00:00] Guard #10 begins shift
            [1518-11-01 00:05] falls asleep
            [1518-11-01 00:25] wakes up
            [1518-11-01 00:30] falls asleep
            [1518-11-01 00:55] wakes up
            [1518-11-01 23:58] Guard #99 begins shift
            [1518-11-02 00:40] falls asleep
            [1518-11-02 00:50] wakes up
            [1518-11-03 00:05] Guard #10 begins shift
            [1518-11-03 00:24] falls asleep
            [1518-11-03 00:29] wakes up
            [1518-11-04 00:02] Guard #99 begins shift
            [1518-11-04 00:36] falls asleep
            [1518-11-04 00:46] wakes up
            [1518-11-05 00:03] Guard #99 begins shift
            [1518-11-05 00:45] falls asleep
            [1518-11-05 00:55] wakes up
        ",
        );

        let guard = find_guard(&guard_sleep_schedule);

        assert_eq!(240, guard);
    }

    #[test]
    fn test2() {
        let guard_sleep_schedule = process_input(
            r"
            [1518-11-01 00:00] Guard #10 begins shift
            [1518-11-01 00:05] falls asleep
            [1518-11-01 00:25] wakes up
            [1518-11-01 00:30] falls asleep
            [1518-11-01 00:55] wakes up
            [1518-11-01 23:58] Guard #99 begins shift
            [1518-11-02 00:40] falls asleep
            [1518-11-02 00:50] wakes up
            [1518-11-03 00:05] Guard #10 begins shift
            [1518-11-03 00:24] falls asleep
            [1518-11-03 00:29] wakes up
            [1518-11-04 00:02] Guard #99 begins shift
            [1518-11-04 00:36] falls asleep
            [1518-11-04 00:46] wakes up
            [1518-11-05 00:03] Guard #99 begins shift
            [1518-11-05 00:45] falls asleep
            [1518-11-05 00:55] wakes up
        ",
        );

        let guard = find_guard_2(&guard_sleep_schedule);

        assert_eq!(4455, guard);
    }
}
