use std::{fs::File, io::{self, BufRead}, collections::HashSet};

struct Forest {
    _data: Vec<u8>,

    line_length: usize
}

impl Forest {
    fn from_file(path: &str) -> Forest {
        let fp = File::open(path).unwrap();
        let buf = io::BufReader::new(fp);
        let mut lines = buf.lines().map(|l| l.unwrap()).peekable();
        let line_length = lines.peek().unwrap().len();
        
        let mut forest = Forest { _data: vec![], line_length: line_length };

        for line in lines {
            forest._data.extend(line.as_bytes().iter().map(|c| *c - ('0' as u8)));
        }

        forest
    }

    fn index_from_coord(&self, x: usize, y: usize) -> usize {
        y * self.line_length + x
    }

    fn coord_from_index(&self, idx: usize) -> (usize, usize) {
        (idx % self.line_length, idx / self.line_length)
    }

    fn _score_direction(&self, start_pos: (usize, usize), direction: (i64, i64)) -> u64 {
        let _start_pos = (start_pos.0 as i64, start_pos.1 as i64);
        let mut pos: (i64, i64) = _start_pos;
        let length = self.line_length as i64;

        let mut prev = None;
        // NOTE: for neg direction, pos will overflow to usize::MAX which should be > to line_length
        while pos.0 >= 0 && pos.0 < length && pos.1 >= 0 && pos.1 < length {

            let idx = self.index_from_coord(pos.0 as usize, pos.1 as usize);
            let value = self._data[idx];

            pos = (pos.0 + direction.0, pos.1 + direction.1);

            if prev.is_none() {
                prev = Some(value);
            }
            else if prev.unwrap() <= value {
                break;
            }
        }

        _start_pos.0.abs_diff(pos.0 - direction.0) + _start_pos.1.abs_diff(pos.1 - direction.1)
    }

    fn score_coord(&self, coord: (usize, usize)) -> u64 {
        let down = self._score_direction(coord, (0, 1));
        let up = self._score_direction(coord, (0, -1));
        let right = self._score_direction(coord, (1, 0));
        let left = self._score_direction(coord, (-1, 0));

        down * up * right * left
    }
}

fn test_line(forest: &Forest, x: usize, horizontal_lookup: bool, reverse_lookup: bool, visible_coords: &mut HashSet<(usize, usize)>) {
    let mut prev = None;
    for idx in 0..forest.line_length {
        let y = match reverse_lookup {
            false => idx,
            true => forest.line_length - idx - 1
        };
        let coord = match horizontal_lookup {
            true => (x, y),
            false => (y, x)
        };
        let index = forest.index_from_coord(coord.0, coord.1);
        if prev.is_none() || forest._data[index] > prev.unwrap() {
            prev = std::cmp::max(prev, Some(forest._data[index]));
            visible_coords.insert(coord);
        }
    }
}

fn main() {
    let forest = Forest::from_file("./src/input.txt");

    let mut visible_coords = HashSet::new();

    for x in 0..forest.line_length {
        test_line(&forest, x, true, false, &mut visible_coords);
        test_line(&forest, x, true, true, &mut visible_coords);
        test_line(&forest, x, false, false, &mut visible_coords);
        test_line(&forest, x, false, true, &mut visible_coords);
    }

    for x in 0..forest.line_length {
        for y in 0..forest.line_length {
            let coord = (x, y);
            if visible_coords.contains(&coord) {
                print!("V");
            }
            else {
                print!(" ");
            }
        }
        println!();
    }

    println!("Found {} visibles", visible_coords.len());

    // Test all possibles
    let mut highest = 0;
    for idx in 0..(forest.line_length*forest.line_length) {
        let coord = forest.coord_from_index(idx);
        let score = forest.score_coord(coord);
        highest = std::cmp::max(score, highest);
    }

    println!("Found coordinate with highest score: {}", highest);
}
