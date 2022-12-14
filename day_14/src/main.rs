use std::{fs::File, io::{self, BufRead}, collections::HashSet, ops::RangeInclusive};


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

fn print_state(y_r: RangeInclusive<i64>, x_r: RangeInclusive<i64>, blocked: &HashSet<(i64, i64)>, sandblocked: &HashSet<(i64, i64)>, c_pos: (i64, i64), floor_level: Option<i64>) {
    let y_r = match floor_level {
        Some(l) => *y_r.start()..=l,
        None => y_r,
    };
    for y in y_r {
        for x in x_r.clone() {
            let pos = (x, y);
            if Some(y) >= floor_level || blocked.contains(&pos) {
                print!("#");
            }
            else if sandblocked.contains(&pos) {
                print!("o");
            }
            else if pos == c_pos {
                print!("~");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
    println!();

}


fn main() {
    let blocked = parse();

    const SAND_ORIGIN: (i64, i64) = (500, 0);

    let (ox, oy) = SAND_ORIGIN;

    let min_x = blocked.iter().map(|(x, _y)| x).min().unwrap().min(&ox).to_owned() - 1;
    let min_y = blocked.iter().map(|(_x, y)| y).min().unwrap().min(&oy).to_owned() - 1;
    let max_x = blocked.iter().map(|(x, _y)| x).max().unwrap().max(&ox).to_owned() + 1;
    let max_y = blocked.iter().map(|(_x, y)| y).max().unwrap().max(&oy).to_owned() + 1;

    println!("{:?} {:?}", (min_x, min_y), (max_x, max_y));

    let mut sandblocked = HashSet::new();

    print_state(min_y..=max_y, min_x..=max_x, &blocked, &sandblocked, (i64::MAX, i64::MAX), None);

    'outer: loop {
        let (mut sx, mut sy) = SAND_ORIGIN;

        loop {
            if sy > max_y {
                break 'outer;
            }

            // print_state(min_y..=max_y, min_x..=max_x, &blocked, &sandblocked, (sx, sy));

            // go down
            let candidates = [
                (sx, sy + 1), // straight down
                (sx - 1, sy + 1), // down-left
                (sx + 1, sy + 1), // down-right
            ];
            match candidates.iter().find(|c| !blocked.contains(&c) && !sandblocked.contains(&c)) {
                Some(c) => (sx, sy) = *c,
                None => {
                    // found resting place
                    sandblocked.insert((sx, sy));
                    break;
                },
            };
        }
    }

    print_state(min_y..=max_y, min_x..=max_x, &blocked, &sandblocked, (i64::MAX, i64::MAX), None);
    println!("Going into the abyss. {}", sandblocked.len());

    // Reset sand
    sandblocked = HashSet::new();
    let floor_level = blocked.iter().map(|(_x, y)| y).max().unwrap().max(&oy).to_owned() + 2;
    'outer: loop {
        let (mut sx, mut sy) = SAND_ORIGIN;

        loop {
            // go down
            let candidates = [
                (sx, sy + 1), // straight down
                (sx - 1, sy + 1), // down-left
                (sx + 1, sy + 1), // down-right
            ];
            match candidates.iter().filter(|(_x, y)| *y < floor_level).find(|c| !blocked.contains(&c) && !sandblocked.contains(&c)) {
                Some(c) => (sx, sy) = *c,
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

    let min_x = sandblocked.iter().map(|(x, _y)| x).min().unwrap().min(&min_x).to_owned() - 1;
    let min_y = sandblocked.iter().map(|(_x, y)| y).min().unwrap().min(&min_y).to_owned();
    let max_x = sandblocked.iter().map(|(x, _y)| x).max().unwrap().max(&max_x).to_owned() + 1;
    let max_y = sandblocked.iter().map(|(_x, y)| y).max().unwrap().max(&max_y).to_owned() + 1;

    print_state(min_y..=max_y, min_x..=max_x, &blocked, &sandblocked, (i64::MAX, i64::MAX), Some(floor_level));
    println!("max count {}", sandblocked.len());
}
