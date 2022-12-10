use std::{fs::File, io::{self, BufRead}, collections::HashSet};
use itertools::Itertools;

fn parse_instruction(line: &String) -> ((i64, i64), i64) {
    let (direction_str, count) = line.split(" ").next_tuple().unwrap();
    let direction = match direction_str {
        "L" => (-1, 0),
        "R" => (1, 0),
        "U" => (0, -1),
        "D" => (0, 1),
        d => panic!("Unexpected direction {}", d),
    };

    (direction, count.parse::<i64>().unwrap())
}

fn resolve(print_edges: Option<&((i64, i64), (i64, i64))>) -> ((i64, i64), (i64, i64)){
    let fp = File::open("./src/input.txt").unwrap();
    let instructions = io::BufReader::new(fp)
        .lines()
        .map(|l| parse_instruction(&l.unwrap()));

    const ROPE_LENGTH: i64 = 1;

    let mut head_pos: (i64, i64) = (0, 0);
    let mut tail_pos = head_pos;

    let mut tail_known_pos = HashSet::<(i64, i64)>::new();
    tail_known_pos.insert(tail_pos);

    let mut board_edges = (head_pos, head_pos);

    if let Some(edges) = print_edges {
        println!("Board edges: {:?}", edges);
    }

    for (direction, count) in instructions {
        for _ in 0..count {
            head_pos = (head_pos.0 + direction.0, head_pos.1 + direction.1);

            board_edges = (
                (board_edges.0.0.min(head_pos.0), board_edges.0.1.min(head_pos.1)),
                (board_edges.1.0.max(head_pos.0), board_edges.1.1.max(head_pos.1))
            );

            let length = (tail_pos.0 - head_pos.0, tail_pos.1 - head_pos.1);
            if length.0.abs() > ROPE_LENGTH || length.1.abs() > ROPE_LENGTH {
                // Move rope tail
                let length = (length.0.clamp(-ROPE_LENGTH, ROPE_LENGTH), length.1.clamp(-ROPE_LENGTH, ROPE_LENGTH));
                tail_pos = (tail_pos.0 - length.0, tail_pos.1 - length.1);
                tail_known_pos.insert(tail_pos);
            }

            if let Some(edges) = print_edges {
                for _ in edges.0.0..=edges.1.0 {
                    print!("=");
                }
                println!();
                for x in edges.0.0..=edges.1.0 {
                    for y in edges.0.1..=edges.1.1 {
                        let print_coord = (x, y);

                        let value;
                        if print_coord == head_pos {
                            value = 'H';
                        }
                        else if print_coord == tail_pos {
                            value = 'T';
                        }
                        else if print_coord == (0, 0) {
                            value = 's';
                        }
                        else {
                            value = '.';
                        }
                        print!("{}", value);
                    }
                    println!();
                }
            }
        }
    }

    if let Some(edges) = print_edges {
        for _ in edges.0.0..=edges.1.0 {
            print!("=");
        }
        println!();
        for x in edges.0.0..=edges.1.0 {
            for y in edges.0.1..=edges.1.1 {
                let print_coord = (x, y);

                let value;
                if tail_known_pos.contains(&print_coord) {
                    value = 'X';
                }
                else {
                    value = '.';
                }
                print!("{}", value);
            }
            println!();
        }
    }

    println!("All touched position: {}", tail_known_pos.len());

    board_edges
}

fn main() {
    let edges = resolve(None);
    // resolve(Some(&edges));
}
