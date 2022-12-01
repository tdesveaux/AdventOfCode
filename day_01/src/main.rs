use std::{fs::File, io::{self, BufRead}};
use std::env;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let res = env::current_dir();
    println!("cwd: {:?}", res);

    let file_path = args.path.as_path();

    println!("Found file {} ({:?})", file_path.display(), file_path.exists());

    if let Err(error) = file_path.try_exists() {
        panic!("File {} not found: {:?}", file_path.display(), error)
    }

    let fp = match File::open(file_path) {
        Ok(fp) => fp,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let lines = io::BufReader::new(fp).lines();

    let mut carried_calories: Vec<i32> = vec![0];

    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(error) => panic!("Problem reading line {:?}", error),
        };
        if line.is_empty() {
            carried_calories.push(0);
        }
        else {
            let calories = match line.parse::<i32>() {
                Ok(c) => c,
                Err(error) => panic!("Problem parsing line {:?}", error),
            };

            *(carried_calories.last_mut().expect("Problem accessing last vec entry")) += calories;
        }
    }

    carried_calories.sort();
    carried_calories.reverse();

    let top_three_total_calories: i32 = (&carried_calories[..3]).iter().sum();
    for calories in &carried_calories[..3] {
        println!("{}", calories);
    }
    println!("Top-three total: {}", top_three_total_calories);
}
