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

    let mut elf_idx = 1;
    let mut current_calories_count = 0;

    let mut top_calories = current_calories_count;
    let mut top_elf_idx = elf_idx;
    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(error) => panic!("Problem reading line {:?}", error),
        };
        if line.is_empty() {
            if current_calories_count > top_calories {
                top_calories = current_calories_count;
                top_elf_idx = elf_idx;
            }

            elf_idx += 1;
            current_calories_count = 0;
        }
        else {
            let calories = match line.parse::<i32>() {
                Ok(c) => c,
                Err(error) => panic!("Problem parsing line {:?}", error),
            };
            current_calories_count += calories;
        }
    }


    println!("Max calories elf {} with {} calories.", top_elf_idx, top_calories);
}
