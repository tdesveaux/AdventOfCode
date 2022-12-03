use std::{fs::File, io::{self, BufRead}, collections::HashSet};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let file_path = args.path.as_path();

    if let Err(error) = file_path.try_exists() {
        panic!("File {} not found: {:?}", file_path.display(), error)
    }

    let fp = File::open(file_path).unwrap();

    let mut total = 0;
    for line in io::BufReader::new(fp).lines() {
        let line = line.unwrap();
        if line.len() % 2 != 0 {
            panic!("Rucksack does not contains same number of items in each compartment");
        }

        let half_index = line.len() / 2;
        let left_compartment: HashSet<u8> = HashSet::from_iter(line[..half_index].bytes());
        let right_compartment: HashSet<u8> = HashSet::from_iter(line[half_index..].bytes());

        let difference_sum: u32 = left_compartment
            .intersection(&right_compartment)
            .map(|c| match *c as char {
                'a'..='z' => *c - ('a' as u8) + 1,
                'A'..='Z' => *c - ('A' as u8) + 27,
                _ => panic!("Unexcpected byte in input")
            } as u32)
            .sum();

        total += difference_sum;
    }

    println!("Total is {}", total);
}
