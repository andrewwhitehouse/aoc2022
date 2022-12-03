enum Choice {
    ROCK=1,
    PAPER=2,
    SCISSORS=3
}

pub fn round_score(opponent_choice: Choice, my_choice: Choice) -> u8 {
    const MY_WIN: u8 = 6;
    const DRAW: u8 = 3;
    const OPPONENT_WIN: u8 = 0;
    let game_score = if my_choice == opponent_choice {
        DRAW
    } else {
        match my_choice {
            Choice::ROCK => if opponent_choice == Choice::SCISSORS { MY_WIN } else { OPPONENT_WIN }
            Choice::PAPER => if opponent_choice == Choice::ROCK { MY_WIN } else { OPPONENT_WIN }
            Choice::SCISSORS => if opponent_choice == Choice::PAPER { MY_WIN } else { OPPONENT_WIN }
        }
    };
    return game_score + my_choice
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_round() {
        assert_eq!(round_score(Choice::ROCK, Choice::PAPER), 8);
    }

    #[test]
    fn test_second_round() {
        assert_eq!(round_score(Choice::PAPER, Choice::ROCK), 1);
    }

    #[test]
    fn test_third_round() {
        assert_eq!(round_score(Choice::SCISSORS, Choice::SCISSORS), 3);
    }
}