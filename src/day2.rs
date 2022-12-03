#[derive(PartialEq)]
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

pub struct PlayerChoice {
    my_choice: Choice,
    opponent_choice: Choice
}

pub fn game_score(player_choices: Vec<PlayerChoice>) -> u16 {
    player_choices.iter().map(|choice| {
        round_score(choice) as u16
    }).sum()
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
}