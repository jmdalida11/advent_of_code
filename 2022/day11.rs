use std::fs;

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multi,
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    added: u64,
    divisible: u64,
    is_old: bool,
    throw_true: usize,
    throw_false: usize,
}

fn parse_numbers(line: &str) -> Vec<u64> {
    let mut v = vec![];
    let mut s = String::new();
    for c in line.chars() {
        if c.is_digit(10) {
            s.push(c);
        } else if !s.is_empty() {
            v.push(s.parse::<u64>().unwrap());
            s.clear();
        }
    }
    if !s.is_empty() {
        v.push(s.parse::<u64>().unwrap());
    }
    v
}

fn parse_monkeys() -> Vec<Monkey> {
    let content = fs::read_to_string("input").expect("File 'input' not found!");

    let mut monkeys: Vec<Monkey> = vec![];
    let mut idx: usize = 0;
    for line in content.lines() {
        let line = line.trim();
        let items = parse_numbers(line);

        if line.starts_with("Monkey") {
            monkeys.push(Monkey {
                id: items[0] as usize,
                items: vec![],
                operation: Operation::Add,
                added: 0,
                divisible: 0,
                is_old: false,
                throw_true: 0,
                throw_false: 0,
            });
            idx = items[0] as usize;
        } else if line.starts_with("Starting") {
            monkeys[idx].items = items;
        } else if line.starts_with("Operation") {
            if line.contains("*") {
                monkeys[idx].operation = Operation::Multi;
            }
            if !items.is_empty() {
                monkeys[idx].added = items[0];
            } else {
                monkeys[idx].is_old = true;
            }
        } else if line.starts_with("Test") {
            monkeys[idx].divisible = items[0];
        } else if line.starts_with("If true") {
            monkeys[idx].throw_true = items[0] as usize;
        } else if line.starts_with("If false") {
            monkeys[idx].throw_false = items[0] as usize;
        }
    }

    return monkeys;
}

fn main() {
    let monkeys = parse_monkeys();

    let lcm_of_divisors = monkeys.iter()
        .map(|monkey| monkey.divisible)
        .fold(1, |acc, x| lcm(acc, x));

    solve(20, monkeys.clone(), lcm_of_divisors, true);
    solve(10000, monkeys, lcm_of_divisors, false);
}

fn solve(rounds: usize, mut monkeys: Vec<Monkey>, lcm: u64, part_1: bool) {
    let monkey_count = monkeys.len();
    let mut inspects = vec![0; monkey_count];

    for _ in 0..rounds {
        for i in 0..monkey_count {
            inspects[monkeys[i].id] += monkeys[i].items.len();
            let items: Vec<u64> = std::mem::take(&mut monkeys[i].items);
            for item in items {
                let worry_level = new_worry_level(item, &monkeys[i], lcm) / if part_1 { 3 } else { 1 };
                let throw_index = if worry_level % monkeys[i].divisible == 0 {
                    monkeys[i].throw_true
                } else {
                    monkeys[i].throw_false
                };
                monkeys[throw_index].items.push(worry_level);
            }
        }
    }

    inspects.sort_by(|a, b| b.cmp(a));
    println!("{}", inspects[0] * inspects[1]);
}

fn new_worry_level(worry_level: u64, monkey: &Monkey, lcm: u64) -> u64 {
    let added = if monkey.is_old { worry_level } else { monkey.added };
    let result = match monkey.operation {
        Operation::Add => worry_level + added,
        Operation::Multi => worry_level * added,
    };
    result % lcm // Reduce modulo LCM
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}
