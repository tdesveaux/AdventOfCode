use std::{fs::File, io::{self, BufRead}, collections::HashSet};

fn manhattan_distance((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> u64 {
    x2.abs_diff(x1) + y2.abs_diff(y1)
}

fn parse() -> impl Iterator<Item = ((i64, i64), (i64, i64))> {
    let fp = File::open("./src/input.txt").unwrap();
    io::BufReader::new(fp)
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            let mut it = l.split_ascii_whitespace();
            it.nth(1); // Skip: Sensor at
            let sensor: (i64, i64) = {
                let (s_x, s_y) = (it.next().unwrap(), it.next().unwrap());
                (s_x[2..s_x.len()-1].parse().unwrap(), s_y[2..s_y.len()-1].parse().unwrap())
            };

            it.nth(3); // closest beacon is at
            let beacon: (i64, i64) = {
                let (b_x, b_y) = (it.next().unwrap(), it.next().unwrap());
                (b_x[2..b_x.len()-1].parse().unwrap(), b_y[2..].parse().unwrap())
            };

            (sensor, beacon)
        })
}

fn part01(sensor_beacon_pairs: impl Iterator<Item = ((i64, i64), (i64, i64))>) {
    const Y_LOOKUP: i64 = 2000000;

    let it = sensor_beacon_pairs
        .map(|((sx, sy), (bx, by))| ((sx, sy), manhattan_distance((sx, sy), (bx, by))))
        .filter(|((_sx, sy), m_d)| {
            let m_d = *m_d as i64;
            let (low, high): (i64, i64) = (sy - m_d, sy + m_d);
            low <= Y_LOOKUP && high >= Y_LOOKUP
        })
        .map(|((sx, sy), m_d)| {
            let y_delta = sy.abs_diff(Y_LOOKUP);
            let x_delta = (m_d - y_delta) as i64;
            (sx-x_delta)..(sx+x_delta)
        });

    let mut hash = HashSet::new();
    for range in it {
        hash.extend(range.into_iter());
    }

    println!("not it {}", hash.len());
}

fn main() {
    let sensor_beacon_pairs =  parse();

    part01(sensor_beacon_pairs);
}
