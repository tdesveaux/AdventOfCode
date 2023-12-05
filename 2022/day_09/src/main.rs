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

    const ROPE_LENGTH: usize = 10;
    const START_POS: (i64, i64) = (0, 0);
    let mut rope = [START_POS; ROPE_LENGTH];

    let mut tail_known_pos = HashSet::<(i64, i64)>::new();
    tail_known_pos.insert(START_POS);

    let mut board_edges = (START_POS, START_POS);

    if let Some(edges) = print_edges {
        println!("Board edges: {:?}", edges);
    }

    for (direction, count) in instructions {

        if let Some(_edges) = print_edges {
            println!();
            println!("==   {} ==", count);
            println!();
        }

        for _ in 0..count {
            // move head
            rope[0] = (rope[0].0 + direction.0, rope[0].1 + direction.1);

            for idx in 1..rope.len() {
                let leader = rope[idx - 1];
                let node = &mut rope[idx];

                let length = (node.0 - leader.0, node.1 - leader.1);
                if length.0.abs() > 1 || length.1.abs() > 1 {
                    // Move rope tail
                    let length = (length.0.clamp(-1, 1), length.1.clamp(-1, 1));
                    *node = (node.0 - length.0, node.1 - length.1);
                }
            }

            tail_known_pos.insert(*rope.last().unwrap());

            board_edges = (
                (board_edges.0.0.min(rope[0].0), board_edges.0.1.min(rope[0].1)),
                (board_edges.1.0.max(rope[0].0), board_edges.1.1.max(rope[0].1))
            );
        }
        if let Some(edges) = print_edges {
            for y in edges.0.1..=edges.1.1 {
                for x in edges.0.0..=edges.1.0 {
                    let print_coord = (x, y);

                    let value: String;
                    if print_coord == rope[0] {
                        value = "H".to_owned();
                    }
                    else if let Some(idx) = rope.iter().position(|e| *e == print_coord) {
                        value = format!("{}", idx);
                    }
                    else if print_coord == (0, 0) {
                        value = "s".to_owned();
                    }
                    else {
                        value = ".".to_owned();
                    }
                    print!("{}", value);
                }
                println!();
            }
        }
}

    if let Some(edges) = print_edges {
        for _ in edges.0.0..=edges.1.0 {
            print!("=");
        }
        println!();
        for y in edges.0.1..=edges.1.1 {
            for x in edges.0.0..=edges.1.0 {
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
