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
}
