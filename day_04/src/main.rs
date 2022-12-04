use std::{fs::File, io::{self, BufRead}};
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

fn part_01(fp: File) {
    let lines = io::BufReader::new(fp).lines();

    let mut complete_overlap = 0;
    for l in lines {
        let ranges = l.unwrap()
            .split([',', '-'])
            .map(|e| e.parse::<i32>().unwrap())
            .tuples()
            .collect::<Vec<(i32, i32)>>();
        if ranges.len() != 2 {
            panic!("unhandled not two ranges");
        }

        let (begin_1, end_1) = ranges[0];
        let (begin_2, end_2) = ranges[1];

        let contains = ((begin_2 - begin_1) * (end_2 - end_1)) <= 0;
        if contains {
            complete_overlap += 1;
        }
    }
    println!("Total contains: {}", complete_overlap);
}

fn part_02(fp: File) {
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
