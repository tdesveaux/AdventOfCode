use std::{fs::File, io::{self, BufRead}};


#[derive(Debug)]
enum Instruction {
    Noop(usize),
    AddX(usize, i64)
}

impl Instruction {
    fn consume(&mut self, state_value: &mut i64) -> bool {
        let remaining_cycles = match self {
            Instruction::Noop(c) => {
                *c -= 1;
                c
            },
            Instruction::AddX(c, v) => {
                *c -= 1;
                if *c == 0 {
                    *state_value += *v;
                }
                c
            }
        };

        // Return wether this op is consumed
        *remaining_cycles == 0
    }
}

struct State {
    reg_x: i64,
    current_cycle: usize,

    _current_instruction: Option<Instruction>,
}

impl State {
    fn exec_cycle(&mut self, instructions: &mut impl Iterator<Item = Instruction>) -> bool {

        if self._current_instruction.is_none() {
            match instructions.next() {
                Some(i) => self._current_instruction = Some(i),
                None => return false // end exec
            };
        }

        if let Some(instruction) = &mut self._current_instruction {
            if instruction.consume(&mut self.reg_x) {
                self._current_instruction = None;
            }
        }

        self.current_cycle += 1;

        true
    }
}

impl Default for State {
    fn default() -> Self {
        Self { reg_x: 1, current_cycle: 1, _current_instruction: None }
    }
}

fn parse_instruction(line: &String) -> Instruction {
    let mut values = line.split(" ");
    let op = values.next().unwrap();
    let instruction = match op {
        "noop" => Instruction::Noop(1),
        "addx" => Instruction::AddX(2, values.next().unwrap().parse().unwrap()),
        unexpected => panic!("Unexpected instruction {}", unexpected)
    };

    instruction
}

fn main() {
    let fp = File::open("./src/input.txt").unwrap();
    let mut instructions = io::BufReader::new(fp)
        .lines()
        .map(|l| parse_instruction(&l.unwrap()));

    let peek_at: [usize; 6] = [20, 60, 100, 140, 180, 220];

    let mut state = State { ..Default::default() };

    let mut sig_strength = 0;
    while state.exec_cycle(instructions.by_ref()) {

        if peek_at.contains(&state.current_cycle) {
            let cycle_sig_strength = (state.current_cycle as i64) * state.reg_x;
            println!("State at cycle {}: {}. Signal strength: {}", state.current_cycle, state.reg_x, cycle_sig_strength);
            sig_strength += cycle_sig_strength;
        }
    }

    println!("Total signal strength {}", sig_strength);
}
