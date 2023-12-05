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

fn get_ranges(line: &String) -> ((i32, i32), (i32, i32)) {
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

fn has_contains((left, right): ((i32, i32), (i32, i32))) -> bool {
    ((right.0 - left.0) * (right.1 - left.1)) <= 0
}

fn has_overlap((left, right): ((i32, i32), (i32, i32))) -> bool {
    left.0 <= right.1 && left.1 >= right.0
}

fn main() {
    let args = Cli::parse();

    let file_path = args.path.as_path();

    if let Err(error) = file_path.try_exists() {
        panic!("File {} not found: {:?}", file_path.display(), error)
    }

    let fp = File::open(file_path).unwrap();
    let filter_func = match args.mode {
        Mode::Part01 => has_contains,
        Mode::Part02 => has_overlap,
    };

    let total = io::BufReader::new(fp)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| filter_func(get_ranges(l)))
        .count();
    println!("Total: {}", total);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_overlap() {
        assert_eq!(has_overlap(((2, 4), (6, 8))), false);
        assert_eq!(has_overlap(((2, 3), (4, 5))), false);

        assert_eq!(has_overlap(((5, 7), (7, 9))), true);
        assert_eq!(has_overlap(((2, 8), (3, 7))), true);
        assert_eq!(has_overlap(((6, 6), (4, 6))), true);
        assert_eq!(has_overlap(((2, 6), (4, 8))), true);
    }
}
