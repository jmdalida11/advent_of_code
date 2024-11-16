use std::fs;

type ValueType = u128;

#[derive(Debug)]
enum Operation {
  Add,
  Multi,
}

#[derive(Debug)]
struct Monkey {
  id: usize,
  items: Vec<ValueType>,
  operation: Operation,
  added: ValueType,
  divisible: ValueType,
  is_old: bool,
  throw_true: usize,
  throw_false: usize,
}

fn parse_numbers(line: &str) -> Vec<ValueType> {
  let mut v = vec![];
  let mut s = String::new();
  for c in line.chars() {
    if c.is_digit(10) {
      s.push(c);
    } else if !s.is_empty() {
      v.push(s.parse::<ValueType>().unwrap());
      s.clear();
    }
  }
  if !s.is_empty() {
    v.push(s.parse::<ValueType>().unwrap());
  }
  v
}

fn main() {
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

    part1(20, monkeys);
}

fn part1(rounds: ValueType, mut monkeys: Vec<Monkey>) {
  let monkey_count = monkeys.len();
    let mut inspects = vec![0; monkey_count];
    
    for _ in 0..rounds {
      for i in 0..monkey_count {
        inspects[monkeys[i].id] += monkeys[i].items.len();
        let items: Vec<ValueType> = std::mem::take(&mut monkeys[i].items);
        for item in items {
          let worry_level = new_worry_level(item, &monkeys[i]) / 3;
          let throw_index = if worry_level % monkeys[i].divisible == 0 {
            monkeys[i].throw_true
          } else {
            monkeys[i].throw_false
          };
          monkeys[throw_index].items.push(worry_level);
        }
        monkeys[i].items.clear();
      }
    }

    inspects.sort_by(|a, b| b.cmp(a));
    println!("{}", inspects[0] * inspects[1]);
}

fn new_worry_level(worry_level: ValueType, monkey: &Monkey) -> ValueType {
  let added = if monkey.is_old { worry_level } else { monkey.added };
  match monkey.operation {
    Operation::Add => worry_level + added,
    Operation::Multi => worry_level * added,
  }
}
