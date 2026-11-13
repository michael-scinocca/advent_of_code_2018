use std::{collections::{HashMap, VecDeque}, fmt::Display};

struct Marbles {
    marbles: VecDeque<u64>,
    current_index: usize,
}

impl Marbles {
    fn new() -> Self {
        Marbles {
            marbles: VecDeque::from([0]),
            current_index: 0,
        }
    }

    fn remove_marble(&mut self) -> u64 {
        for _ in 0..7 {
            if self.current_index == 0 {
                self.current_index = self.marbles.len();
            }

            self.current_index -= 1;
        }

        let removed = self.marbles.remove(self.current_index);

        if self.current_index == self.marbles.len() {
            self.current_index = 0;
        }

        removed.unwrap()
    }

    fn place_marble(&mut self, marble: u64) {
        self.current_index += 1;

        if self.current_index >= self.marbles.len() {
            self.current_index = 0;
        }

        self.current_index += 1;

        if self.current_index == self.marbles.len() {
            self.marbles.push_back(marble);
        } else {
            self.marbles.insert(self.current_index, marble);
        }
    }
}

impl Display for Marbles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, marble) in self.marbles.iter().enumerate() {
            if i == self.current_index {
                write!(f, "({}) ", marble)?
            } else {
                write!(f, " {} ", marble)?
            }
        }

        Ok(())
    }
}
pub fn part1() {
    let high_score = run_game(463, 71787);

    println!("{}", high_score);
}

pub fn part2() {
    let high_score = run_game(463, 71787 * 100);

    println!("{}", high_score);
}

fn run_game(num_players: u32, last_marble: u64) -> u64 {
    let mut player_turn = 0;
    let mut player_scores = HashMap::new();

    let mut marbles = Marbles::new();

    for marble in 1..=last_marble {
        player_turn += 1;

        if player_turn > num_players {
            player_turn = 1;
        }

        if marble % 23 == 0 {
            let score = marble + marbles.remove_marble();

            player_scores
                .entry(player_turn)
                .and_modify(|v| *v += score)
                .or_insert(score);
        } else {
            marbles.place_marble(marble);
        }

        if marble == last_marble {
            break;
        }
    }

    *player_scores.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(9, 25, 32)]
    #[case(10, 1618, 8317)]
    #[case(13, 7999, 146373)]
    #[case(17, 1104, 2764)]
    #[case(21, 6111, 54718)]
    #[case(30, 5807, 37305)]
    fn test1(#[case] num_players: u32, #[case] last_marble: u64, #[case] expected_score: u64) {
        let score = run_game(num_players, last_marble);

        assert_eq!(expected_score, score);
    }
}
