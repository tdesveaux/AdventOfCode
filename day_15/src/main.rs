use std::{fs::File, io::{self, BufRead}, ops::{Range, IndexMut, RangeInclusive, Sub, Mul, Div, Add}, collections::{HashMap}};

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

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn cross(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div<Self> for Point {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Segment {
    p1: Point,
    p2: Point,
}

impl Segment {
    fn slope(&self) -> (f64, f64) {
        (
            (self.p2.x - self.p1.x).clamp(-1.0, 1.0),
            (self.p2.y - self.p1.y).clamp(-1.0, 1.0),
        )
    }

    fn fast_parallel(&self, other: &Segment) -> bool {
        let s_self = self.slope();
        let s_other = other.slope();

        // Since we know the slopes are for x/y = -1 or 1
        s_self.0 * s_self.1 == s_other.0 * s_other.1
    }

    // from: https://stackoverflow.com/a/565282
    // but we assume the two lines aren't parallel
    fn intersect_point(&self, s2: &Segment) -> Option<Point> {
        let p = self.p1;
        let q = s2.p1;

        let r = self.p2 - p;
        let s = s2.p2 - q;

        if r.cross(&s) == 0.0 {
            return None;
        }

        let t = (q - p).cross(&(s / r.cross(&s)));
        let u = (q - p).cross(&(r / r.cross(&s)));

        let accepted_range = 0.0..=1.0;
        if !accepted_range.contains(&t) || !accepted_range.contains(&u) {
            return None;
        }

        Some(Point {
            x: p.x + t * r.x,
            y: p.y + t * r.y,
        })
    }
}

fn part02(sensor_beacon_pairs: impl Iterator<Item = ((i64, i64), (i64, i64))>) {
    const BOUNDS: RangeInclusive<i64> = 0..=4000000;

    let sensors = sensor_beacon_pairs
        .map(|((sx, sy), (bx, by))| {
            let m_d = manhattan_distance((sx, sy), (bx, by));

            (
                Point {
                    x: sx as f64,
                    y: sy as f64,
                },
                m_d
            )
        }).collect::<Vec<(Point, u64)>>();

    // only points on the outside edge of the sensor range can be valid
    let mut candidates = sensors.iter()
        .map(|(sensor, m_d)| {
            let m_d = *m_d as f64;
            let top = Point { x: sensor.x, y: sensor.y - m_d - 1.0 };
            let bottom = Point { x: sensor.x, y: sensor.y + m_d + 1.0 };
            let left = Point { x: sensor.x - m_d - 1.0, y: sensor.y };
            let right = Point { x: sensor.x + m_d + 1.0, y: sensor.y };

            [
                Segment { p1: left, p2: top },
                Segment { p1: top, p2: right },
                Segment { p1: right, p2: bottom },
                Segment { p1: bottom, p2: left },
            ]
        })
        .flatten()
        .collect::<Vec<Segment>>();

    let slope_to_bit = |s: (f64, f64)| {
        match (s.0 as i64, s.1 as i64) {
            (-1, -1) => 0,
            (1, -1) => 1,
            (-1, 1) => 2,
            (1, 1) => 3,
            _ => panic!()
        }
    };

    let mut points = HashMap::new();

    while let Some(candidate) = candidates.pop() {
        let bit = slope_to_bit(candidate.slope());

        for other in &candidates {
            if candidate.fast_parallel(other) {
                continue;
            }
            if let Some(point) = candidate.intersect_point(other) {
                let (x, y) = (point.x as i64, point.y as i64);
                if BOUNDS.contains(&x) && BOUNDS.contains(&y) {
                    // println!("{:?} | {:?} -> {:?}", candidate, other, point);
                    let entry = points.entry((x, y)).or_insert([false; 4]);
                    entry[bit] = true;

                    let bit = slope_to_bit(other.slope());
                    entry[bit] = true;
                }
            }
        }
    }

    let (b_low, b_high) = (*BOUNDS.start(), *BOUNDS.end());
    for (point, intersects) in points {
        let x_on_edge = if point.0 == b_low || point.0 == b_high { 1 } else { 0 };
        let y_on_edge = if point.1 == b_low || point.1 == b_high { 1 } else { 0 };
        // Bonus if on edge as it might not have outer / we don't care about outside sensor

        let intersect_all_sides = intersects.iter().filter(|&e| *e).count() + x_on_edge + y_on_edge >= 4;
        if !intersect_all_sides {
            continue;
        }
        if sensors.iter().any(|(s, d)| manhattan_distance((s.x as i64, s.y as i64), point) <= *d) {
            // in range of sensor, skip
            continue;
        }

        println!("Found {:?} for {}", point, point.0 * b_high + point.1);
    }
}

fn main() {
    part01(parse());
    part02(parse());
}
