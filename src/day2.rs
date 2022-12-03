#[derive(PartialEq)]
pub enum Choice {
    ROCK,
    PAPER,
    SCISSORS
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
    let choice_score = match my_choice {
        Choice::ROCK => 1,
        Choice::PAPER => 2,
        Choice::SCISSORS => 3
    };
    return game_score + choice_score;
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    #[test]
    fn test_rock_paper() {
        assert_eq!(round_score(Choice::ROCK, Choice::PAPER), 8);
    }

    #[test]
    fn test_paper_rock() {
        assert_eq!(round_score(Choice::PAPER, Choice::ROCK), 1);
    }

    #[test]
    fn test_both_scissors() {
        assert_eq!(round_score(Choice::SCISSORS, Choice::SCISSORS), 6);
    }

    #[test]
    fn test_both_rock() {
        assert_eq!(round_score(Choice::ROCK, Choice::ROCK), 4);
    }

    #[test]
    fn test_both_paper() {
        assert_eq!(round_score(Choice::PAPER, Choice::PAPER), 5);
    }

    #[test]
    fn test_scissors_rock() {
        assert_eq!(round_score(Choice::SCISSORS, Choice::ROCK), 7);
    }

    #[test]
    fn test_scissors_paper() {
        assert_eq!(round_score(Choice::SCISSORS, Choice::PAPER), 2);
    }
}