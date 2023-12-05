use std::{fs::File, io::{self, BufRead}, borrow::{BorrowMut}};
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

fn print_cargo(cargo: &Vec<Vec<char>>) {
    let highest_len = cargo.iter().map(|s| s.len()).max().unwrap();
    let default: String = "   ".to_string();
    for i in (0..highest_len).rev() {
        let line = cargo.iter()
            .map(|s| if s.len() > i { format!("[{}]", s[i]) } else { default.to_owned() })
            .join(" ");
        println!("{}", line);
    }

    println!("{}", cargo.iter().enumerate().map(|(idx, _)| format!(" {:?} ", idx)).join(" "));
}

fn part(fp: File, over_9000_mode: bool) {
    let mut lines = io::BufReader::new(fp)
        .lines()
        .map(Result::unwrap)
        .peekable();

    let mut cargo: Vec<Vec<char>> = vec![];
    // One item is 3 chars + 1 sep = 4
    // len + 1 for stripped line break
    let cargo_columns_count = (lines.peek().unwrap().len() + 1) / 4;
    cargo.resize_with(cargo_columns_count, Default::default);

    for line in lines.borrow_mut() {
        if line.is_empty() {
            // Separator
            break;
        }
        // initialize
        let chars_it = line.char_indices().skip(1);
        for (pos, c) in chars_it.step_by(4) {
            let pos = (pos - 1) / 4;
            match c {
                'A'..='Z' => cargo[pos].insert(0, c),
                _ => ()
            };
        }
    }
    println!("Starting cargo:");
    print_cargo(&cargo);

    for line in lines {
        let (quantity, origin, destination) = line
            .split(" ") // split on words
            .skip(1) // skip 'move'
            .step_by(2) // skip 'from' and 'to'
            .map(|e| e.parse::<usize>().unwrap())
            .collect_tuple().unwrap();

        if origin != destination {
            // 0-based indexes
            let (origin, destination) = (origin - 1, destination - 1);
            // Unsafe due to multiple mut borrow on cargo
            unsafe {
                let origin_stack: *mut Vec<char> = cargo.get_mut(origin).unwrap();
                let destination_stack: *mut Vec<char> = cargo.get_mut(destination).unwrap();

                let drain_idx = (*origin_stack).len() - quantity;
                let drained = (*origin_stack).drain(drain_idx..);
                if over_9000_mode {
                    (*destination_stack).extend(drained.rev());
                }
                else {
                    (*destination_stack).extend(drained);
                }
            }
        }
        println!("{}", line);
        print_cargo(&cargo);
    }

    println!("result top line: {}", cargo.iter().map(|v| v.last().unwrap()).join(""));
}

fn main() {
    let args = Cli::parse();

    let file_path = args.path.as_path();

    if let Err(error) = file_path.try_exists() {
        panic!("File {} not found: {:?}", file_path.display(), error)
    }

    let fp = File::open(file_path).unwrap();
    match args.mode {
        Mode::Part01 => part(fp, false),
        Mode::Part02 => part(fp, false),
    };
}
