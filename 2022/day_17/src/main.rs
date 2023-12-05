use std::{fs::File, io::{self, BufRead}};

enum Move {
    Left,
    Right,
}

fn parse_moves() -> Vec<Move> {
    let fp = File::open("./src/input-test.txt").unwrap();
    io::BufReader::new(fp)
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            l.chars().map(|c| {
                match c {
                    '<' => Move::Left,
                    '>' => Move::Right,
                    _ => panic!()
                }
            }).collect::<Vec<Move>>()
        })
        .flatten()
        .collect()
}

type Shape = Vec<Vec<u8>>;

const WIDTH: usize = 7;
const X_START: usize = 2;
const Y_START_OFFSET: usize = 3;

fn valid_pos(state: &Vec<u8>, pos: (usize, usize), shape: &Shape) -> bool {

    for y in 0..shape.len() {
        let shape_line = &shape[shape.len() - 1 - y];

        for x in 0..shape_line.len() {
            if shape_line[x] == 0 {
                continue;
            }

            if x + pos.0 >= WIDTH {
                return false;
            }

            let state_pos = (pos.0 + x, pos.1 + y);
            let mask = 0b1 << state_pos.0;
            if state.len() > state_pos.1 && (state[state_pos.1] & mask) != 0 {
                return false;
            }
        }
    }

    true
}

fn part01(moves: &Vec<Move>, shapes: &[Shape; 5]) {
    let stopped_rock_count = 2022;

    let mut shape_idx: usize = 0;
    let mut move_idx: usize = 0;

    let mut state : Vec<u8> = vec![];

    for _rock_idx in 0..stopped_rock_count {

        let mut rock_pos = (X_START, state.len() + Y_START_OFFSET);
        let shape = &shapes[shape_idx];

        loop {
            // do move
            {
                let tentative_x = match moves[move_idx] {
                    Move::Left if rock_pos.0 > 0 => rock_pos.0 - 1,
                    Move::Right if rock_pos.0 < WIDTH => rock_pos.0 + 1,
                    _ => rock_pos.0,
                };
                move_idx = (move_idx + 1) % moves.len();

                if tentative_x != rock_pos.0 && valid_pos(&state, (tentative_x, rock_pos.1), shape) {
                    rock_pos.0 = tentative_x;
                }
            }

            // do down
            {
                if rock_pos.1 == 0 {
                    break;
                }
                let tentative_y = rock_pos.1 - 1;
                if valid_pos(&state, (rock_pos.0, tentative_y), shape) {
                    rock_pos.1 = tentative_y;
                }
                else {
                    break;
                }
            }
        }

        // Commit shape
        {
            for y in 0..shape.len() {
                let shape_line = &shape[shape.len() - 1 - y];

                let state_y = rock_pos.1 + y;
                for x in 0..shape_line.len() {
                    if shape_line[x] != 0 {
                        while state.len() <= state_y {
                            state.push(0);
                        }

                        state[state_y] |= 0b1 << rock_pos.0 + x;
                    }
                }
            }
        }

        shape_idx = (shape_idx + 1) % shapes.len();
    }

    println!("Highest: {}", state.len());
}

fn main() {
    let first: Shape = vec![
        vec![1, 1, 1, 1],
    ];

    let second: Shape = vec![
        vec![0, 1, 0],
        vec![1, 1, 1],
        vec![0, 1, 0],
    ];

    let third: Shape = vec![
        vec![0, 0, 1],
        vec![0, 0, 1],
        vec![1, 1, 1],
    ];

    let fourth: Shape = vec![
        vec![1],
        vec![1],
        vec![1],
        vec![1],
    ];

    let fifth: Shape = vec![
        vec![1, 1],
        vec![1, 1],
    ];

    let shapes: [Shape; 5] = [first, second, third, fourth, fifth];

    let moves = parse_moves();
    part01(&moves, &shapes);
}
