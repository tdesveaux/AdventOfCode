use std::{fs::File, io::{self, BufRead}, str::FromStr, fmt::{Display, self}};
use clap::Parser;

#[derive(clap::ValueEnum, Clone)]
enum Mode {
    Part01,
    Part02,
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,

    #[arg(value_enum)]
    mode: Mode,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    fn get_matching_choice(&self, other: Choice) -> Choice {
        let other_v = (other as i8) - 1; // other => [0, 2]
        let self_v = ((*self as i8) / 3) - 1; // self => [-1, 1]
        let result = (other_v + self_v).rem_euclid(3);
        Choice::try_from(result + 1).unwrap()
    }
}

impl FromStr for Outcome {

    type Err = ();

    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _   => Err(()),
        }
    }
}

impl Choice {
    fn get_outcome(&self, other: Choice) -> Outcome {
        match ((*self as i8) - (other as i8) + 4) % 3  {
            0 => Outcome::Lose,
            1 => Outcome::Draw,
            2 => Outcome::Win,
            outcome => panic!("Unexpected outcome {}", outcome)
        }
    }

    fn get_score_for_round(&self, other: Choice) -> i8 {
        (*self as i8) + (self.get_outcome(other) as i8)
    }
}

impl TryFrom<i8> for Choice {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::Rock as i8 => Ok(Self::Rock),
            x if x == Self::Paper as i8 => Ok(Self::Paper),
            x if x == Self::Scissor as i8 => Ok(Self::Scissor),
            unhandled => panic!("Unhandled conversion from {} to Choice", unhandled),
        }
    }
}

impl FromStr for Choice {

    type Err = ();

    fn from_str(input: &str) -> Result<Choice, Self::Err> {
        match input {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissor),
            _   => Err(()),
        }
    }
}

impl Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Rock => "Rock",
            Self::Paper => "Paper",
            Self::Scissor => "Scissor"
        })
    }
}

impl Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Draw => "Draw",
            Self::Lose => "Lose",
            Self::Win => "Win"
        })
    }
}

// A X : Rock
// B Y : Paper
// C Z : Scissor

// Score: My selection + Outcome
// With selection: Rock: 1, Paper: 2, Scissor: 3
// Outcome: List: 0, Draw: 3, Won: 6

fn part_01(fp: File) {
    let mut total_score: i32 = 0;
    for line in io::BufReader::new(fp).lines() {
        let line = line.unwrap();
        if line.len() != 3 || &line[1..2] != " " {
            panic!("line does not have proper format");
        }
        let other_choice = Choice::from_str(&line[..1]).unwrap();
        let my_choice = Choice::from_str(&line[2..3]).unwrap();

        let round_score = my_choice.get_score_for_round(other_choice);

        total_score += round_score as i32;

        println!("{} vs {}, score: {}", other_choice, my_choice, round_score);
    }

    println!("Final score: {}", total_score);
}

// A: Rock
// B: Paper
// C: Scissor

// X: Lose
// Y: Draw
// Z: Win

// Score: My selection + Outcome
// With selection: Rock: 1, Paper: 2, Scissor: 3
// Outcome: List: 0, Draw: 3, Won: 6

fn part_02(fp: File) {

    let mut total_score: i32 = 0;
    for line in io::BufReader::new(fp).lines() {
        let line = line.unwrap();
        if line.len() != 3 || &line[1..2] != " " {
            panic!("line does not have proper format");
        }
        let other_choice = Choice::from_str(&line[..1]).unwrap();
        let outcome = Outcome::from_str(&line[2..3]).unwrap();

        let found_choice = outcome.get_matching_choice(other_choice);

        let round_score = (found_choice as i8) + (outcome as i8);

        total_score += round_score as i32;

        println!("{} with {} => {}", outcome, other_choice, found_choice);
    }

    println!("Final score: {}", total_score);
}

fn main() {
    let args = Cli::parse();

    let file_path = args.path.as_path();

    if let Err(error) = file_path.try_exists() {
        panic!("File {} not found: {:?}", file_path.display(), error)
    }

    let fp = File::open(file_path).unwrap();

    match args.mode {
        Mode::Part01 => part_01(fp),
        Mode::Part02 => part_02(fp),
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Outcome::Lose.get_matching_choice(Choice::Rock), Choice::Scissor);
        assert_eq!(Outcome::Lose.get_matching_choice(Choice::Paper), Choice::Rock);
        assert_eq!(Outcome::Lose.get_matching_choice(Choice::Scissor), Choice::Paper);

        assert_eq!(Outcome::Draw.get_matching_choice(Choice::Rock), Choice::Rock);
        assert_eq!(Outcome::Draw.get_matching_choice(Choice::Paper), Choice::Paper);
        assert_eq!(Outcome::Draw.get_matching_choice(Choice::Scissor), Choice::Scissor);

        assert_eq!(Outcome::Win.get_matching_choice(Choice::Rock), Choice::Paper);
        assert_eq!(Outcome::Win.get_matching_choice(Choice::Paper), Choice::Scissor);
        assert_eq!(Outcome::Win.get_matching_choice(Choice::Scissor), Choice::Rock);
    }
}
