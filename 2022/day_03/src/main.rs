use std::{fs::File, io::{self, BufRead}, collections::HashSet};
use clap::Parser;
use itertools::Itertools;

#[derive(clap::ValueEnum, Clone)]
enum Mode {
    Part01,
    Part02,
}

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,

    #[arg(value_enum)]
    mode: Mode,
}

fn score_item(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => panic!("Unexcpected byte in input")
    }
}

fn part_01(fp: File) {
    let mut total = 0;
    for line in io::BufReader::new(fp).lines() {
        let line = line.unwrap();
        if line.len() % 2 != 0 {
            panic!("Rucksack does not contains same number of items in each compartment");
        }

        let half_index = line.len() / 2;
        let left_compartment: HashSet<char> = HashSet::from_iter(line[..half_index].chars());
        let right_compartment: HashSet<char> = HashSet::from_iter(line[half_index..].chars());

        let difference_sum: u32 = left_compartment
            .intersection(&right_compartment)
            .map(|c| score_item(*c))
            .sum();

        total += difference_sum;
    }

    println!("Total is {}", total);
}

fn part_02(fp: File) {
    const GROUP_SIZE: usize = 3;

    let lines = io::BufReader::new(fp)
        .lines()
        .map(|l| l.unwrap());

    let mut total: u32 = 0;
    for group in &lines.chunks(GROUP_SIZE) {

        let chunk_intersection = group
            .map(|c| HashSet::<char>::from_iter(c.chars()))
            .reduce(|accum, item| &accum & &item)
            .expect("Failed to find common tag");

        if chunk_intersection.len() != 1 {
            panic!("Found more than one common item");
        }

        total += chunk_intersection.iter().map(|c| score_item(*c)).sum::<u32>();
    }

    println!("Total is {}", total);
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
