use std::{io::{self, BufRead}, fs::File, borrow::{BorrowMut}, collections::VecDeque, str::FromStr, fmt::Display, cell::RefCell};

enum Operator {
    Mult,
    Add
}

impl Operator {
    fn compute(&self, left: i64, right: i64) -> i64 {
        match self {
            Self::Mult => left * right,
            Self::Add => left + right,
        }
    }
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Operator::Mult),
            "+" => Ok(Operator::Add),
            _ => Err(())
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Add => "increased",
            Self::Mult => "multiplied",
        })
    }
}

enum OperatorPart {
    Old,
    Number(i64)
}

impl OperatorPart {
    fn value(&self, old: i64) -> i64 {
        match self {
            Self::Old => old,
            Self::Number(n) => *n
        }
    }
}

impl FromStr for OperatorPart {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(OperatorPart::Old),
            n => Ok(OperatorPart::Number(n.parse().unwrap()))
        }
    }
}

impl Display for OperatorPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Old => write!(f, "itself"),
            Self::Number(n) => write!(f, "{}", n),
        }
    }
}

struct InRoundPrint;
impl InRoundPrint {
    const ENABLED: bool = false;

    fn print(s: String) {
        if Self::ENABLED {
            print!("{}", s);
        }
    }

    fn println(s: String) {
        if Self::ENABLED {
            println!("{}", s);
        }
    }
}

struct InterRoundPrint;
impl InterRoundPrint {
    const ENABLED: bool = true;

    fn print(s: String) {
        if Self::ENABLED {
            print!("{}", s);
        }
    }

    fn println(s: String) {
        if Self::ENABLED {
            println!("{}", s);
        }
    }
}

struct Monkey {
    items: VecDeque<i64>,

    operators: Vec<Operator>,
    operator_parts: Vec<OperatorPart>,

    test_value: i64,
    test_ok_monkey_idx: usize,
    test_err_monkey_idx: usize,

    inpection_count: usize,
}

impl Monkey {
    fn resolve_ops(&self, old_value: i64) -> i64 {
        let mut parts_it = self.operator_parts.iter();
        let mut operator_it = self.operators.iter();

        InRoundPrint::print(format!("    Worry level is"));

        let mut new = parts_it.next().unwrap().value(old_value);
        while let Some(op) = operator_it.next() {
            let part = parts_it.next().unwrap();

            InRoundPrint::print(format!(" {} by {} ", op, part));

            new = op.compute(new, part.value(old_value));
        }

        InRoundPrint::println(format!("to {}.", new));

        new
    }
}

impl Default for Monkey {
    fn default() -> Self {
        Self {
            items: Default::default(),
            operators: Default::default(),
            operator_parts: Default::default(),
            test_value: Default::default(),
            test_ok_monkey_idx: Default::default(),
            test_err_monkey_idx: Default::default(),
            inpection_count: 0
        }
    }
}

fn parse_monkey(monkey_lines:  &mut impl Iterator<Item = String>) -> Monkey {

    let mut monkey = Monkey::default();

    // Ignore 'header' "Monkey {digit}:"
    monkey_lines.next();

    monkey.items = monkey_lines.next().unwrap()
                        .trim().strip_prefix("Starting items: ").unwrap()
                        .split(", ")
                        .map(|i| i.parse().unwrap())
                        .collect();

    {
        let operation_str = monkey_lines.next().unwrap();
        let mut operation = operation_str
                                .trim()
                                .strip_prefix("Operation: new = ").unwrap()
                                .split(" ");

        while let Some(e) = operation.by_ref().borrow_mut().next() {
            if monkey.operator_parts.len() != 0 && monkey.operator_parts.len() / 2 == monkey.operators.len() {
                monkey.operators.push(Operator::from_str(e).unwrap());
            }
            else {
                monkey.operator_parts.push(OperatorPart::from_str(e).unwrap());
            }
        }
    }

    {
        monkey.test_value = monkey_lines.next().unwrap()
                            .trim().strip_prefix("Test: divisible by ").unwrap()
                            .parse().unwrap();

        monkey.test_ok_monkey_idx = monkey_lines.next().unwrap()
                                        .trim().strip_prefix("If true: throw to monkey ").unwrap()
                                        .parse().unwrap();

        monkey.test_err_monkey_idx = monkey_lines.next().unwrap()
                                        .trim().strip_prefix("If false: throw to monkey ").unwrap()
                                        .parse().unwrap();
    }

    for l in monkey_lines {
        if !l.is_empty() {
            println!("unexpected monkey data {}", l);
        }
    }

    monkey
}

fn parse() -> Vec<RefCell<Monkey>> {
    let fp = File::open("./src/input.txt").unwrap();
    let mut lines = io::BufReader::new(fp)
        .lines()
        .map(Result::unwrap)
        .peekable();

    let mut monkeys = vec![];
    while let Some(peeked) = lines.borrow_mut().peek() {
        if peeked.starts_with("Monkey ") {
            let monkey = parse_monkey(lines.by_ref().take_while(|l| !l.is_empty()).by_ref());
            monkeys.push(RefCell::new(monkey));
        }
    }

    monkeys
}

fn play_round(monkeys: &mut Vec<RefCell<Monkey>>) {
    for monkey_idx in 0..monkeys.len() {
        InRoundPrint::println(format!("Monkey {}:", monkey_idx));

        let mut monkey = monkeys[monkey_idx].borrow_mut();

        while let Some(mut item) = monkey.items.pop_front() {
            InRoundPrint::println(format!("  Monkey inspects an item with a worry level of {}.", item));

            monkey.inpection_count += 1;

            item = monkey.resolve_ops(item);

            item = item / 3;
            InRoundPrint::println(format!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", item));

            let test_result = item % monkey.test_value == 0;
            InRoundPrint::println(format!("    Current worry level is{} divisible by {}.", (if test_result { "" } else {" not"}), monkey.test_value));
            let next_monkey_idx = match test_result {
                true => monkey.test_ok_monkey_idx,
                false => monkey.test_err_monkey_idx
            };

            InRoundPrint::println(format!("    Item with worry level {} is thrown to monkey {}.", item, next_monkey_idx));

            monkeys[next_monkey_idx].borrow_mut().items.push_back(item);
        }
    }
}

fn main() {
    let mut monkeys = parse();

    for round in 1..=20 {
        play_round(&mut monkeys);

        InterRoundPrint::println(format!("After round {}, the monkeys are holding items with these worry levels:", round));
        for (idx, m) in monkeys.iter().enumerate() {
            InterRoundPrint::println(format!("  Monkey {}: {}", idx, m.borrow().items.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", ")));
        }
        InterRoundPrint::println(String::new());
    }

    monkeys.sort_by(|l, r| r.borrow().inpection_count.cmp(&l.borrow().inpection_count));

    let monkey_business = monkeys[..2].iter()
        .map(|m| m.borrow().inpection_count)
        .reduce(|acc, item| acc * item).unwrap();

    println!("{}. That's some major monkey business", monkey_business);
}
