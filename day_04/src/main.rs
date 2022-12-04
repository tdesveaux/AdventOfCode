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

fn get_ranges(line: String) -> ((i32, i32), (i32, i32)) {
    let ranges = line
        .split([',', '-'])
        .map(|e| e.parse::<i32>().unwrap())
        .tuples()
        .collect::<Vec<(i32, i32)>>();
    if ranges.len() != 2 {
        panic!("unhandled not two ranges");
    }

    (ranges[0], ranges[1])
}

fn has_overlap(left: (i32, i32), right: (i32, i32)) -> bool {
    left.0 <= right.1 && left.1 >= right.0
}

fn part_01(fp: File) {
    let lines = io::BufReader::new(fp).lines();

    let mut complete_overlap = 0;
    for l in lines {
        let ((begin_1, end_1), (begin_2, end_2)) = get_ranges(l.unwrap());

        let contains = ((begin_2 - begin_1) * (end_2 - end_1)) <= 0;
        if contains {
            complete_overlap += 1;
        }
    }
    println!("Total contains: {}", complete_overlap);
}

fn part_02(fp: File) {
    let lines = io::BufReader::new(fp).lines();

    let mut overlap = 0;
    for l in lines {
        let (left, right) = get_ranges(l.unwrap());

        if has_overlap(left, right) {
            overlap += 1;
        }
    }
    println!("Total overlap: {}", overlap);
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
    fn test_has_overlap() {
        assert_eq!(has_overlap((2, 4), (6, 8)), false);
        assert_eq!(has_overlap((2, 3), (4, 5)), false);

        assert_eq!(has_overlap((5, 7), (7, 9)), true);
        assert_eq!(has_overlap((2, 8), (3, 7)), true);
        assert_eq!(has_overlap((6, 6), (4, 6)), true);
        assert_eq!(has_overlap((2, 6), (4, 8)), true);
    }
}
