use std::collections::HashMap;

#[derive(PartialEq, Eq)]
enum TaskState {
    Pending,
    Running,
    Complete,
}

struct TaskMap<'a>(HashMap<&'a str, (TaskState, Vec<&'a str>)>);

impl<'a> TaskMap<'a> {
    fn new() -> Self {
        TaskMap(HashMap::new())
    }

    fn get_next_task(&mut self) -> Option<String> {
        if self.0.is_empty() {
            return None;
        }

        let mut next_tasks = self
            .0
            .iter_mut()
            .filter(|(_, s)| s.0 == TaskState::Pending && s.1.is_empty())
            .collect::<Vec<_>>();

        next_tasks.sort_by_key(|t| t.0);

        if let Some(next_task) = next_tasks.first_mut() {
            Some(next_task.0.to_string())
        } else {
            None
        }
    }

    fn set_task_running(&mut self, task: &str) {
        self.0.get_mut(task).unwrap().0 = TaskState::Running;
    }

    fn complete_task(&mut self, task: &str) {
        if let Some(task) = self.0.get_mut(task) {
            task.0 = TaskState::Complete;
        }

        for gates in self.0.values_mut() {
            gates.1.retain(|g| *g != task);
        }
    }

    fn run(&mut self) -> String {
        let mut tasks = String::new();

        while let Some(task) = self.get_next_task() {
            self.set_task_running(&task);
            self.complete_task(&task);

            tasks.push_str(&task);
        }

        tasks
    }

    fn run_parallel(&mut self, base_task_time: u32, worker_count: usize) -> u32 {
        let mut total_time = 0;

        let mut workers: Vec<Option<(String, u32)>> = vec![None; worker_count];

        loop {
            while let Some(next_task) = self.get_next_task() {
                if let Some(next_worker) = workers.iter_mut().find(|w| w.is_none()) {
                    let task_time =
                        (next_task.as_bytes()[0] - "A".as_bytes()[0] + 1) as u32 + base_task_time;

                    self.set_task_running(&next_task);

                    *next_worker = Some((next_task, task_time));
                } else {
                    break;
                }
            }

            for worker in workers.iter_mut() {
                if let Some(active_worker) = worker {
                    active_worker.1 -= 1;

                    if active_worker.1 == 0 {
                        self.complete_task(&active_worker.0);

                        *worker = None;
                    }
                }
            }

            if self.0.iter().all(|t| t.1.0 == TaskState::Complete)
                && workers.iter().all(|w| w.is_none())
            {
                break;
            }

            total_time += 1;
        }

        total_time + 1
    }
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day7.txt").unwrap();

    let mut task_map = process_input(&input);

    println!("{}", task_map.run());
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day7.txt").unwrap();

    let mut task_map = process_input(&input);

    println!("{}", task_map.run_parallel(60, 5));
}

fn process_input(input: &str) -> TaskMap<'_> {
    let mut task_map = TaskMap::new();

    for line in input
        .trim()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
    {
        let mut parts = line.split_whitespace();

        let gate = parts.nth(1).unwrap();
        let task = parts.nth(5).unwrap();

        task_map
            .0
            .entry(task)
            .and_modify(|gates| gates.1.push(gate))
            .or_insert_with(|| (TaskState::Pending, vec![gate]));

        if !task_map.0.contains_key(gate) {
            task_map.0.insert(gate, (TaskState::Pending, Vec::new()));
        }
    }

    task_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = r"
            Step C must be finished before step A can begin.
            Step C must be finished before step F can begin.
            Step A must be finished before step B can begin.
            Step A must be finished before step D can begin.
            Step B must be finished before step E can begin.
            Step D must be finished before step E can begin.
            Step F must be finished before step E can begin.
        ";

        let mut task_map = process_input(input);

        assert_eq!("CABDFE", task_map.run());
    }

    #[test]
    fn test2() {
        let input = r"
            Step C must be finished before step A can begin.
            Step C must be finished before step F can begin.
            Step A must be finished before step B can begin.
            Step A must be finished before step D can begin.
            Step B must be finished before step E can begin.
            Step D must be finished before step E can begin.
            Step F must be finished before step E can begin.
        ";

        let mut task_map = process_input(input);

        assert_eq!(15, task_map.run_parallel(0, 2));
    }
}
