use std::{fs::File, io::{self, BufRead}, fmt::Display};

enum Element {
    Value(u64),
    Array(Vec<Element>),
}

impl Element {

    fn get_array(s: &str) -> &str {
        let mut bracket_count = 1;
        let mut end_idx = 1;
        let mut cursor = s.chars().skip(1);
        while bracket_count > 0 {
            end_idx += 1;
            match cursor.next().unwrap() {
                '[' => bracket_count += 1,
                ']' => bracket_count -= 1,
                _ => (),
            }
        }

        &s[0..end_idx]
    }

    fn get_element(s: &str) -> &str {
        let end_idx = s.chars().position(|c| c == ',' || c == ']').unwrap_or(s.len());
        &s[..end_idx]
    }

    fn parse_array(s: &str) -> Element {
        let mut elements = vec![];
        let mut array = &s[1..s.len()-1];
        while array.len() > 0 {
            let (new_element, skip_len) = match array.chars().nth(0).unwrap() {
                ',' => { array = &array[1..]; continue; },
                '[' => {
                    let sub_str = Self::get_array(array);
                    (Self::parse_array(sub_str), sub_str.len())
                },
                _ => {
                    let sub_str = Self::get_element(array);
                    (Element::Value(sub_str.parse().unwrap()), sub_str.len())
                }
            };
            elements.push(new_element);

            array = &array[skip_len..];
        }

        Element::Array(elements)
    }

    fn from_str(s: &str) -> Element {
        Self::parse_array(s)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::Array(a) => {
                write!(f, "[").unwrap();
                if !a.is_empty() {
                    for idx in 0..a.len()-1 {
                        write!(f, "{},", a[idx]).unwrap();
                    }
                    write!(f, "{}", a.last().unwrap()).unwrap();
                }

                write!(f, "]")
            }
        }
    }
}

fn parse_pair(pair_iterator: &mut impl Iterator<Item = String>) -> (Element, Element) {

    let left = Element::from_str(&pair_iterator.next().unwrap()[..]);
    let right = Element::from_str(&pair_iterator.next().unwrap()[..]);

    if let Some(x) = pair_iterator.next() {
        panic!("Unexpected value {x}");
    }

    (left, right)
}

fn print_prefix(depth: usize) {
    for _ in 0..depth {
        print!("  ");
    }
}

enum Comparison {
    Less,
    Equal,
    More,
}

fn compare_order(left: &Element, right: &Element, depth: usize) -> Comparison {
    print_prefix(depth);
    println!("- Compare {} vs {}", left, right);

    if let (Element::Value(left_v), Element::Value(right_v)) = (&left, &right) {
        if left_v == right_v {
            return Comparison::Equal;
        }
        else if left_v < right_v {
            print_prefix(depth+1);
            println!("- Left side is smaller, so inputs are in the right order");
            return Comparison::Less;
        }
        else {
            print_prefix(depth+1);
            println!("- Right side is smaller, so inputs are not in the right order");
            return Comparison::More;
        }
    }
    else if let (Element::Array(left_array), Element::Array(right_array)) = (&left, &right) {
        let (mut left_it, mut right_it) = (left_array.iter(), right_array.iter());

        loop {
            let (n_left, n_right) = (left_it.next(), right_it.next());

            if let (Some(v_left), Some(v_right)) = (n_left, n_right) {
                match compare_order(v_left, v_right, depth+1) {
                    Comparison::Less => return Comparison::Less,
                    Comparison::Equal => (),
                    Comparison::More => return Comparison::More,
                }
            }
            else if n_left.is_none() && n_right.is_none() {
                // comsumed whole array
                return Comparison::Equal;
            }
            else if n_left.is_none() {
                print_prefix(depth+1);
                println!("- Left side ran out of items, so inputs are in the right order");
                return Comparison::Less;
            }
            else if n_right.is_none() {
                print_prefix(depth+1);
                println!("- Right side ran out of items, so inputs are not in the right order");
                return Comparison::More;
            }
        }
    }
    else {
        if let Element::Value(left_v) = &left {
            let new_left = Element::Array(vec![Element::Value(*left_v)]);
            print_prefix(depth+1);
            println!("- Mixed types; convert left to {} and retry comparison", new_left);
            return compare_order(&new_left, right, depth+1)
        }
        else if let Element::Value(right_v) = &right {
            let new_right = Element::Array(vec![Element::Value(*right_v)]);
            print_prefix(depth+1);
            println!("- Mixed types; convert right to {} and retry comparison", new_right);
            return compare_order(left, &new_right, depth+1)
        }
    }

    panic!("SHOULD NOT HAPPEN");
}

fn main() {
    let fp = File::open("./src/input.txt").unwrap();
    let mut lines = io::BufReader::new(fp)
        .lines()
        .map(Result::unwrap)
        .peekable();

    let mut correct_pair_sum = 0;
    let mut pair_count = 0;
    while let Some(peeked) = &lines.peek() {
        if !peeked.is_empty() {
            let (left, right) = parse_pair(lines.by_ref().take_while(|l| !l.is_empty()).by_ref());

            pair_count += 1;
            println!("== Pair {} ==", pair_count);
            match compare_order(&left, &right, 0) {
                Comparison::Less => correct_pair_sum += pair_count,
                _ => ()
            };

            println!();
        }
    }

    println!("Total correct pair sum {}", correct_pair_sum);
}
