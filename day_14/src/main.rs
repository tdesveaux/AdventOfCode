use std::{fs::File, io::{self, BufRead}, collections::HashSet};

fn parse() -> HashSet<(i64, i64)> {
    const SEPARATOR: &str = " -> ";
    let fp = File::open("./src/input.txt").unwrap();
    let lines = io::BufReader::new(fp)
        .lines()
        .map(Result::unwrap);

    let mut blocked = HashSet::new();

    for item in lines {
        let mut entries = item
            .split(SEPARATOR)
            .map(|e| e.split_once(',').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()));

        let (mut p_x, mut p_y): (i64, i64) = entries.next().unwrap();
        while let Some(current) = entries.next() {
            let (dx, dy) = (
                (current.0 - p_x).clamp(-1, 1),
                (current.1 - p_y).clamp(-1, 1)
            );
            if dx != 0 && dy != 0 {
                panic!("Unhandled vector delta");
            }
            while (p_x, p_y) != current {
                blocked.insert((p_x, p_y));

                (p_x, p_y) = (p_x + dx, p_y + dy);
            }
            blocked.insert((p_x, p_y));
        }
    }

    blocked
}

fn main() {
    let blocked = parse();

    const SAND_ORIGIN: (i64, i64) = (500, 0);

    let max_y = blocked.iter().map(|(_x, y)| y).max().unwrap().max(&SAND_ORIGIN.1).to_owned();

    let mut sandblocked = HashSet::new();

    let mut passage = vec![];
    'outer: loop {
        let (mut sx, mut sy) = passage.pop().unwrap_or(SAND_ORIGIN);

        loop {
            if sy >= max_y {
                break 'outer;
            }

            // go down
            let candidates = [
                (sx, sy + 1), // straight down
                (sx - 1, sy + 1), // down-left
                (sx + 1, sy + 1), // down-right
            ];
            match candidates.iter().find(|&c| !blocked.contains(c) && !sandblocked.contains(c)) {
                Some(c) => {
                    passage.push((sx, sy));
                    (sx, sy) = *c
                }
                None => {
                    // found resting place
                    sandblocked.insert((sx, sy));
                    break;
                },
            };
        }
    }

    println!("Going into the abyss. {}", sandblocked.len());

    let floor_level = max_y + 2;

    'outer: loop {
        let (mut sx, mut sy) = passage.pop().unwrap_or(SAND_ORIGIN);

        loop {
            // go down
            let candidates = [
                (sx, sy + 1), // straight down
                (sx - 1, sy + 1), // down-left
                (sx + 1, sy + 1), // down-right
            ];
            match candidates.iter().filter(|(_x, y)| *y < floor_level).find(|c| !blocked.contains(&c) && !sandblocked.contains(&c)) {
                Some(c) => {
                    passage.push((sx, sy));
                    (sx, sy) = *c
                }
                None => {
                    // found resting place
                    sandblocked.insert((sx, sy));
                    if (sx, sy) == SAND_ORIGIN {
                        break 'outer;
                    }
                    break;
                },
            };
        }
    }

    println!("max count {}", sandblocked.len());
}
