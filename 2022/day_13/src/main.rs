use std::{fs::File, io::{self, BufRead}, fmt::Display};

#[derive(Clone)]
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

fn compare_order(left: &Element, right: &Element) -> std::cmp::Ordering {

    let tmp_ref; // ref holder
    let (left_, right_) = match (&left, &right) {
        (Element::Value(l), Element::Array(_r)) => {
            tmp_ref = Element::Array(vec![Element::Value(*l)]);
            (&tmp_ref, right)
        },
        (Element::Array(_l), Element::Value(r)) => {
            tmp_ref = Element::Array(vec![Element::Value(*r)]);
            (left, &tmp_ref)
        }
        _ => (left, right)
    };

    match (&left_, &right_) {
        (Element::Value(l), Element::Value(r)) => l.cmp(r),
        (Element::Array(l), Element::Array(r)) => {
            let (mut l_it, mut r_it) = (l.iter(), r.iter());

            loop {
                match (l_it.next(), r_it.next()) {
                    (Some(v_l), Some(v_r)) => {
                        match compare_order(v_l, v_r) {
                            std::cmp::Ordering::Equal => (),
                            o => return o,
                        }
                    },
                    (None, None) => return std::cmp::Ordering::Equal, // comsumed both arrays
                    (None, Some(_r)) => return std::cmp::Ordering::Less,
                    (Some(_l), None) => return std::cmp::Ordering::Greater,
                };
            };
        },
        _ => panic!("Should not reach")
    }
}

fn main() {
    {
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
                match compare_order(&left, &right) {
                    std::cmp::Ordering::Less => correct_pair_sum += pair_count,
                    _ => ()
                };
            }
        }

        println!("Total correct pair sum {}", correct_pair_sum);
    }

    {
        let fp = File::open("./src/input.txt").unwrap();
        let mut elements: Vec<Element> = io::BufReader::new(fp)
            .lines()
            .map(Result::unwrap)
            .filter(|l| !l.is_empty())
            .map(|l| Element::from_str(&l[..]))
            .collect();

        let divider_packets = [
            Element::from_str("[[2]]"),
            Element::from_str("[[6]]"),
        ];

        elements.extend(divider_packets.iter().map(|x| x.clone()));

        elements.sort_by(|l, r| compare_order(l, r));

        let decoder_key: usize =elements
            .iter()
            .enumerate()
            .filter(|(_idx, e)| (&divider_packets).iter().any(|d| compare_order(e, d) == std::cmp::Ordering::Equal))
            .map(|(idx, _e)| idx+1)
            .product();

        println!("Decoder key is {}", decoder_key);
     }
}
