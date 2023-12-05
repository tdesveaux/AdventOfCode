use std::{fs::File, io::{self, Read}, collections::VecDeque};
use clap::Parser;

#[derive(clap::ValueEnum, Clone)]
enum Mode {
    Part01,
    Part02,
}

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,

    #[arg(value_enum, default_value_t=Mode::Part02)]
    mode: Mode,
}

fn all_uniq(buffer: &VecDeque<u8>) -> bool {
    for i in 0..buffer.len() {
        for j in i+1..buffer.len() {
            if buffer[i] == buffer[j] {
                return false;
            }
        }
    }

    return true;
}

fn find_marker(buf_reader: &mut io::BufReader<File>, packet_size: usize) {
    let mut buffer = [0; 1];
    let mut candidate = VecDeque::<u8>::with_capacity(packet_size);
    let mut index = 0;
    while buf_reader.read_exact(&mut buffer).is_ok() {
        index += 1;
        while candidate.len() >= packet_size {
            candidate.pop_front().unwrap();
        }
        candidate.push_back(buffer[0]);
        if candidate.len() >= packet_size && all_uniq(&candidate) {
            println!("marker index: {}", index);
            return;
        }
    }

    panic!("failed to find message");
}

fn main() {
    let args = Cli::parse();

    let file_path = args.path.as_path();

    if let Err(error) = file_path.try_exists() {
        panic!("File {} not found: {:?}", file_path.display(), error)
    }

    let fp = File::open(file_path).unwrap();
    let mut buf = io::BufReader::new(fp);
    match args.mode {
        Mode::Part01 => find_marker(&mut buf, 4),
        Mode::Part02 => find_marker(&mut buf, 14),
    };
}
