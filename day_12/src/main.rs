use std::{fs::File, io::{self, BufRead}, collections::{VecDeque, BinaryHeap}, ops::Index, cmp::Reverse};

enum Area {
    Start,
    End,
    Height(u8),
}

impl Area {
    const  _base_value: u8 = 'a' as u8;

    fn from_char(c: char) -> Area {
        match c {
            'S' => Area::Start,
            'E' => Area::End,
            c => Area::Height(c as u8 - Self::_base_value),
        }
    }

    fn get_elevation(&self) -> u8 {
        match self {
            Area::End => 'z' as u8 - Area::_base_value,
            Area::Start => 'a' as u8 - Area::_base_value,
            Area::Height(e) => *e
        }
    }
}

struct Map {
    _data: Vec<Area>,

    width: usize,
    height: usize,

    start_idx: usize,
    end_idx: usize,
}

impl Map {
    fn from_file(path: &str) -> Map {
        let fp = File::open(path).unwrap();
        let buf = io::BufReader::new(fp);
        let mut lines = buf.lines().map(|l| l.unwrap()).peekable();
        let line_length = lines.peek().unwrap().len();

        let mut map = Map { _data: vec![], width: line_length, height: Default::default(), start_idx: Default::default(), end_idx: Default::default() };

        for line in lines {
            map._data.reserve(line_length);
            for c in line.chars() {
                let a = Area::from_char(c);
                match a {
                    Area::Start => map.start_idx = map._data.len(),
                    Area::End => map.end_idx = map._data.len(),
                    _ => (),
                };
                map._data.push(a);
            }
        }

        map.height = map._data.len() / map.width;

        map
    }

    fn index_from_coord(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn coord_from_index(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn coord_neighbors(&self, x: usize, y: usize) -> [Option<(usize, usize)>; 4] {
        [
            if x > 0 { Some((x - 1, y)) } else { None },
            if x + 1 < self.width { Some((x + 1, y)) } else { None },

            if y > 0 { Some((x, y - 1)) } else { None },
            if y + 1 < self.height { Some((x, y + 1)) } else { None },
        ]
    }

    fn can_go(&self, current_idx: usize, candidate_idx: usize) -> bool {
        let candidate_elevation = self._data[candidate_idx].get_elevation();

        let current_elevation: u8 = self._data[current_idx].get_elevation();

        if current_elevation >= candidate_elevation {
            return true;
        }
        (candidate_elevation - current_elevation) <= 1
    }
}

fn main() {
    let map = Map::from_file("./src/input.txt");

    for y in 0..map.height {

        for x in 0..map.width {
            let idx = map.index_from_coord(x, y);
            match map._data[idx] {
                Area::Start => print!("SS"),
                Area::End => print!("EE"),
                Area::Height(e) => print!("{e:0width$}", width=2),
            }
            print!("|");
        }

        println!();
    }

    println!();

    // init weight to 'inf'->u64 max
    let mut weights: Vec<u64> = vec![u64::MAX; map._data.len()];

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), map.end_idx));

    const NODE_DISTANCE: u64 = 1;

    while let Some((Reverse(weight), idx)) = heap.pop() {
        if weights[idx] <= weight {
            continue;
        }
        weights[idx] = weight;

        let (x, y) = map.coord_from_index(idx);
        for n in map.coord_neighbors(x, y) {
            if let Some((n_x, n_y)) = n {
                let n_idx = map.index_from_coord(n_x, n_y);

                if !map.can_go(n_idx, idx) {
                    continue;
                }

                let new_weight = weight + NODE_DISTANCE;
                if new_weight < weights[n_idx] {
                    heap.push((Reverse(new_weight), n_idx));
                }
            }
        }

    }

    for y in 0..map.height {
        for x in 0..map.width {
            let idx = map.index_from_coord(x, y);
            let weight = weights[idx];
            if weight != u64::MAX {
                print!("{:0width$}|", weight, width=2);
            }
            else {
                print!("NA|");
            }
        }

        println!();
    }
    println!();

    println!("Weight: {}", weights[map.start_idx]);

    let print_best_starting_weight = (0..map._data.len()).filter(|idx| {
        match map._data[*idx] {
            Area::Start => true,
            Area::Height(e) => e == 0,
            _ => false
        }
    }).map(|e| weights[e])
    .min().unwrap();
    println!("Best starting weight: {print_best_starting_weight}");
}
