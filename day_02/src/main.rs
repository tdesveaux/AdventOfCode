use std::{fs::File, io::{self, BufRead}, str::FromStr, fmt::{Display, self}};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[derive(Clone, Copy)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
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

impl FromStr for Choice {

    type Err = ();

    fn from_str(input: &str) -> Result<Choice, Self::Err> {
        match input {
            "A" | "X"   => Ok(Choice::Rock),
            "B" | "Y"   => Ok(Choice::Paper),
            "C" | "Z"   => Ok(Choice::Scissor),
            _           => Err(()),
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

// A X : Rock
// B Y : Paper
// C Z : Scissor

// Score: My selection + Outcome
// With selection: Rock: 1, Paper: 2, Scissor: 3
// Outcome: List: 0, Draw: 3, Won: 6

fn main() {
    let args = Cli::parse();

    let file_path = args.path.as_path();

    if let Err(error) = file_path.try_exists() {
        panic!("File {} not found: {:?}", file_path.display(), error)
    }

    let mut total_score: i32 = 0;
    let fp = File::open(file_path).unwrap();
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
