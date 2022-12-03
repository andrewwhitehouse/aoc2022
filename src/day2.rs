#[derive(PartialEq, Debug)]
pub enum Choice {
    ROCK,
    PAPER,
    SCISSORS
}

pub fn round_score(player_choice: &PlayerChoice) -> u8 {
    const MY_WIN: u8 = 6;
    const DRAW: u8 = 3;
    const OPPONENT_WIN: u8 = 0;
    let game_score = if player_choice.my_choice == player_choice.opponent_choice {
        DRAW
    } else {
        match player_choice.my_choice {
            Choice::ROCK => if player_choice.opponent_choice == Choice::SCISSORS { MY_WIN } else { OPPONENT_WIN }
            Choice::PAPER => if player_choice.opponent_choice == Choice::ROCK { MY_WIN } else { OPPONENT_WIN }
            Choice::SCISSORS => if player_choice.opponent_choice == Choice::PAPER { MY_WIN } else { OPPONENT_WIN }
        }
    };
    let choice_score = match player_choice.my_choice {
        Choice::ROCK => 1,
        Choice::PAPER => 2,
        Choice::SCISSORS => 3
    };
    return game_score + choice_score;
}

#[derive(PartialEq, Debug)]
pub struct PlayerChoice {
    my_choice: Choice,
    opponent_choice: Choice
}

pub fn game_score(player_choices: Vec<PlayerChoice>) -> u16 {
    player_choices.iter().map(|choice| {
        round_score(choice) as u16
    }).sum()
}

pub fn parse(input: String) -> Vec<PlayerChoice> {
    let mut player_choices = Vec::new();
    for line in input.trim_end().split("\n") {
        let mut single_round_choices = line.split(" ");
        let opponent_choice = match single_round_choices.next() {
            Some("A") => Choice::ROCK,
            Some("B") => Choice::PAPER,
            Some("C") => Choice::SCISSORS,
            Some(&_) => todo!(),
            None => todo!()
        };
        let my_choice = match single_round_choices.next() {
            Some("X") => Choice::ROCK,
            Some("Y") => Choice::PAPER,
            Some("Z") => Choice::SCISSORS,
            Some(&_) => todo!(),
            None => todo!()
        };
        player_choices.push(PlayerChoice{opponent_choice: opponent_choice, my_choice: my_choice});
    }
    player_choices
}

#[derive(PartialEq, Debug)]
pub enum Outcome {
    WIN,
    LOSE,
    DRAW
}

#[derive(PartialEq, Debug)]
pub struct StrategyPart2 {
    opponent_choice: Choice,
    desired_outcome: Outcome
}

pub fn parse_part2(input: String) -> Vec<StrategyPart2> {
    let mut outcomes = Vec::new();
    for line in input.trim_end().split("\n") {
        let mut single_strategy = line.split(" ");
        let opponent_choice = match single_strategy.next() {
            Some("A") => Choice::ROCK,
            Some("B") => Choice::PAPER,
            Some("C") => Choice::SCISSORS,
            Some(&_) => todo!(),
            None => todo!()
        };
        let desired_outcome = match single_strategy.next() {
            Some("X") => Outcome::LOSE,
            Some("Y") => Outcome::DRAW,
            Some("Z") => Outcome::WIN,
            Some(&_) => todo!(),
            None => todo!()
        };
        outcomes.push(StrategyPart2{opponent_choice: opponent_choice, desired_outcome: desired_outcome});
    }
    outcomes
}

pub fn required_choice(opponent_choice: Choice, desired_outcome: Outcome) -> Choice {
    match desired_outcome {
        Outcome::LOSE => {
            match opponent_choice {
                Choice::ROCK => Choice::SCISSORS,
                Choice::PAPER => Choice::ROCK,
                Choice::SCISSORS => Choice::PAPER
            }
        },
        Outcome::DRAW => opponent_choice,
        Outcome::WIN => {
            match opponent_choice {
                Choice::ROCK => Choice::PAPER,
                Choice::PAPER => Choice::SCISSORS,
                Choice::SCISSORS => Choice::ROCK
            }
        }
    }
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    #[test]
    fn test_rock_paper() {
        assert_eq!(round_score(&PlayerChoice{opponent_choice: Choice::ROCK, my_choice: Choice::PAPER}), 8);
    }

    #[test]
    fn test_paper_rock() {
        assert_eq!(round_score(&PlayerChoice{opponent_choice: Choice::PAPER, my_choice: Choice::ROCK}), 1);
    }

    #[test]
    fn test_both_scissors() {
        assert_eq!(round_score(&PlayerChoice{opponent_choice: Choice::SCISSORS, my_choice: Choice::SCISSORS}), 6);
    }

    #[test]
    fn test_both_rock() {
        assert_eq!(round_score(&PlayerChoice{opponent_choice: Choice::ROCK, my_choice: Choice::ROCK}), 4);
    }

    #[test]
    fn test_both_paper() {
        assert_eq!(round_score(&PlayerChoice{opponent_choice: Choice::PAPER, my_choice: Choice::PAPER}), 5);
    }

    #[test]
    fn test_scissors_rock() {
        assert_eq!(round_score(&PlayerChoice{opponent_choice: Choice::SCISSORS, my_choice: Choice::ROCK}), 7);
    }

    #[test]
    fn test_scissors_paper() {
        assert_eq!(round_score(&PlayerChoice{opponent_choice: Choice::SCISSORS, my_choice: Choice::PAPER}), 2);
    }

    #[test]
    fn test_game_score() {
        let player_choices = vec!(PlayerChoice{opponent_choice: Choice::ROCK, my_choice: Choice::PAPER},
                                 PlayerChoice{opponent_choice: Choice::PAPER, my_choice: Choice::ROCK},
                                 PlayerChoice{opponent_choice: Choice::SCISSORS, my_choice: Choice::SCISSORS});
        assert_eq!(game_score(player_choices), 15);
    }

    #[test]
    fn test_parse() {
        let input = "A Y\nB X\nC Z";
        let expected = vec!(PlayerChoice{opponent_choice: Choice::ROCK, my_choice: Choice::PAPER},
                            PlayerChoice{opponent_choice: Choice::PAPER, my_choice: Choice::ROCK},
                            PlayerChoice{opponent_choice: Choice::SCISSORS, my_choice: Choice::SCISSORS});
        let result = parse(String::from(input));
        assert_eq!(expected.len(), result.len());
        for i in 0..result.len() {
            assert_eq!(expected[i], result[i]);
        }
    }

    #[test]
    fn test_parse_part2() {
        let input = "A Y\nB X\nC Z";
        let expected = vec!(StrategyPart2{opponent_choice: Choice::ROCK, desired_outcome: Outcome::DRAW},
                            StrategyPart2{opponent_choice: Choice::PAPER, desired_outcome: Outcome::LOSE},
                            StrategyPart2{opponent_choice: Choice::SCISSORS, desired_outcome: Outcome::WIN});
        let result = parse_part2(String::from(input));
        assert_eq!(expected.len(), result.len());
        for i in 0..result.len() {
            assert_eq!(expected[i], result[i]);
        }
    }

    #[test]
    fn test_required_choice_for_outcome() {
        assert_eq!(required_choice(Choice::ROCK, Outcome::WIN), Choice::PAPER);
        assert_eq!(required_choice(Choice::ROCK, Outcome::DRAW), Choice::ROCK);
        assert_eq!(required_choice(Choice::ROCK, Outcome::LOSE), Choice::SCISSORS);
        assert_eq!(required_choice(Choice::PAPER, Outcome::WIN), Choice::SCISSORS);
        assert_eq!(required_choice(Choice::PAPER, Outcome::DRAW), Choice::PAPER);
        assert_eq!(required_choice(Choice::PAPER, Outcome::LOSE), Choice::ROCK);
        assert_eq!(required_choice(Choice::SCISSORS, Outcome::WIN), Choice::ROCK);
        assert_eq!(required_choice(Choice::SCISSORS, Outcome::DRAW), Choice::SCISSORS);
        assert_eq!(required_choice(Choice::SCISSORS, Outcome::LOSE), Choice::PAPER);
    }
}