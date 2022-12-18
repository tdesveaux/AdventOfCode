use std::{fs::File, io::{self, BufRead}, ops::{Range, IndexMut, RangeInclusive}};

fn manhattan_distance((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> u64 {
    x2.abs_diff(x1) + y2.abs_diff(y1)
}

fn parse() -> impl Iterator<Item = ((i64, i64), (i64, i64))> {
    let fp = File::open("./src/input.txt").unwrap();
    io::BufReader::new(fp)
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            let mut it = l.strip_prefix("Sensor at x=").unwrap();
            let sensor: (i64, i64) = {
                let sx_str;
                (sx_str, it) = it.split_at(it.find(',').unwrap());
                it = it.strip_prefix(", y=").unwrap();
                let sy_str;
                (sy_str, it) = it.split_at(it.find(':').unwrap());
                (sx_str.parse().unwrap(), sy_str.parse().unwrap())
            };

            it = it.strip_prefix(": closest beacon is at x=").unwrap();

            let beacon: (i64, i64) = {
                let bx_str;
                (bx_str, it) = it.split_at(it.find(',').unwrap());
                it = it.strip_prefix(", y=").unwrap();
                let by_str = it;
                (bx_str.parse().unwrap(), by_str.parse().unwrap())
            };

            (sensor, beacon)
        })
}

fn has_overlap(left: &Range<i64>, right: &Range<i64>) -> bool {
    left.start <= right.end && left.end >= right.start
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

    let mut vec: Vec<Range<i64>> = it.collect();

    // merge ranges
    let mut i = 0;
    while i < vec.len() {
        let range = vec[i].clone();

        if let Some(idx) = vec[i+1..].iter().position(|e| has_overlap(e, &range) || has_overlap(&range, e)) {
            let other = vec.remove(i + 1 + idx);
            *vec.index_mut(i) = other.start.min(range.start)..other.end.max(range.end);

            i = 0;
        }
        else {
            i += 1;
        }
    }

    println!("not it {}", vec.iter().map(|r| r.start.abs_diff(r.end)).sum::<u64>());
}

fn part02(sensor_beacon_pairs: impl Iterator<Item = ((i64, i64), (i64, i64))>) {
    const _BOUNDS: RangeInclusive<i64> = 0..=4_000_000;

    // only points on the outside edge of the sensor range can be valid
    let _candidates = sensor_beacon_pairs
        .map(|((sx, sy), (bx, by))| {
            let m_d = manhattan_distance((sx, sy), (bx, by)) as i64;

            let top = (sx, sy - m_d - 1);
            let bottom = (sx, sy + m_d + 1);
            let left = (sx - m_d - 1, sy);
            let right = (sx + m_d + 1, sy);

            [
                (left, top),
                (top, right),
                (right, bottom),
                (bottom, left),
            ]
        });

    // TODO: Check intersections between 'candidates' segments
    // The target point should be an intersection between
    // segments comming from all side
}

fn main() {
    part01(parse());
    part02(parse());
}
