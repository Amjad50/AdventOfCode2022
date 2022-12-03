use std::{env, fs, io};

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_char(c: char) -> Option<RPS> {
        match c {
            'A' | 'X' => Some(RPS::Rock),
            'B' | 'Y' => Some(RPS::Paper),
            'C' | 'Z' => Some(RPS::Scissors),
            _ => None,
        }
    }

    fn to_loser(self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }

    fn to_winner(self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
}

fn add_to_score(score: &mut u32, mine: RPS, opponent: RPS) {
    let i_win = match (mine, opponent) {
        (RPS::Rock, RPS::Scissors) => true,
        (RPS::Paper, RPS::Rock) => true,
        (RPS::Scissors, RPS::Paper) => true,
        _ => false,
    };
    let draw = opponent == mine;

    let choice_score = match mine {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };
    let win_score = if i_win {
        6
    } else if draw {
        3
    } else {
        0
    };

    *score += choice_score + win_score;
}

fn main() -> io::Result<()> {
    let file_data = fs::read_to_string(env::args().nth(1).expect("Input file arg"))?;

    let mut p1_score = 0;
    let mut p2_score = 0;

    file_data.lines().for_each(|line| {
        let mut split_chars = line.chars();
        // first char is the opponent choice char
        let opponent_char = split_chars.next().unwrap();
        // space separator
        assert!(split_chars.next().unwrap() == ' ');
        // my choice
        let mine_char = split_chars.next().unwrap();
        assert!(split_chars.next().is_none());

        let opponent = RPS::from_char(opponent_char).unwrap();
        let mine = RPS::from_char(mine_char).unwrap();

        add_to_score(&mut p1_score, mine, opponent);

        let p2_mine = match mine_char {
            'X' => opponent.to_loser(),
            'Y' => opponent,
            'Z' => opponent.to_winner(),
            _ => unreachable!(),
        };
        add_to_score(&mut p2_score, p2_mine, opponent);
    });

    println!("Part1: score: {}", p1_score);
    println!("Part2: score: {}", p2_score);

    Ok(())
}
