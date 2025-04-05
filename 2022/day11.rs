use std::{ fs, fmt };
use std::ops::{Add, Mul, Rem, Div};
use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    Add,
    Multi,
}

#[derive(Debug, Clone)]
struct BigInt {
    value: Vec<u8>,
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut value = "".to_string();
        for c in self.value.iter().rev() {
            value.push((*c + b'0') as char);
        }
        write!(f, "{}", value)
    } 
}

impl BigInt {
    fn new(v: String) -> Self {
        let mut value = vec![];
        for c in v.chars().rev() {
            value.push(c.to_digit(10).unwrap() as u8);
        }
        return Self {
            value,
        };
    }

    fn add(&self, other: &BigInt) -> BigInt {
        let mut result = self.value.clone();
        let mut remainder = 0;

        for i in 0..result.len() {
            if other.value.len() <= i {
                let new_value = result[i] + remainder;
                result[i] = new_value % 10;
                remainder = new_value / 10;
            } else {
                let new_value = result[i] + other.value[i] + remainder;
                result[i] = new_value % 10;
                remainder = new_value / 10;
            }
        }

        if other.value.len() > result.len() {
            for i in result.len()..other.value.len() {
                let new_value = other.value[i] + remainder;
                result.push(new_value % 10);
                remainder = new_value / 10;
            }
        }

        if remainder > 0 {
            result.push(remainder);
        }

        BigInt { value: result }
    }

    fn multi(&self, other: &BigInt) -> BigInt {
        let mut result = vec![0; self.value.len() + other.value.len()];

        for (i, &self_digit) in self.value.iter().enumerate() {
            let mut carry = 0;
            for (j, &other_digit) in other.value.iter().enumerate() {
                let sum = result[i + j] + self_digit * other_digit + carry;
                result[i + j] = sum % 10;
                carry = sum / 10;
            }
            if carry > 0 {
                let mut k = i + other.value.len();
                result[k] += carry;
                while result[k] >= 10 {
                    result[k + 1] += result[k] / 10;
                    result[k] %= 10;
                    k += 1;
                }
            }
        }

        // Remove leading zeros
        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        BigInt { value: result }
    }

    fn modulo(&self, other: &BigInt) -> BigInt {
        let mut remainder = BigInt { value: vec![0] };

        for &digit in self.value.iter().rev() {
            remainder.value.insert(0, digit);

            // Remove leading zeros in remainder
            while remainder.value.len() > 1 && remainder.value.last() == Some(&0) {
                remainder.value.pop();
            }

            // Perform division to find the remainder
            while remainder >= *other {
                remainder = remainder.subtract(other);
            }
        }

        remainder
    }

    fn subtract(&self, other: &BigInt) -> BigInt {
        let mut result = self.value.clone();
        let mut borrow = 0;

        for (i, &other_digit) in other.value.iter().enumerate() {
            let mut diff = result[i] as i32 - other_digit as i32 - borrow;
            if diff < 0 {
                diff += 10;
                borrow = 1;
            } else {
                borrow = 0;
            }
            result[i] = diff as u8;
        }

        for i in other.value.len()..result.len() {
            if borrow == 0 {
                break;
            }
            let mut diff = result[i] as i32 - borrow;
            if diff < 0 {
                diff += 10;
                borrow = 1;
            } else {
                borrow = 0;
            }
            result[i] = diff as u8;
        }

        // Remove leading zeros
        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        BigInt { value: result }
    }

    fn to_usize(&self) -> usize {
        self.value
            .iter()
            .rev()
            .fold(0, |acc, &digit| acc * 10 + digit as usize)
    }
}

// Add comparison operators for BigInt
impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.value.len() != other.value.len() {
            return self.value.len().cmp(&other.value.len()).into();
        }
        self.value.iter().rev().cmp(other.value.iter().rev()).into()
    }
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, other: BigInt) -> BigInt {
        (&self).add(&other)
    }
}

impl Mul for BigInt {
    type Output = BigInt;

    fn mul(self, other: BigInt) -> BigInt {
        (&self).multi(&other)
    }
}

impl Rem for BigInt {
    type Output = BigInt;

    fn rem(self, other: BigInt) -> BigInt {
        (&self).modulo(&other)
    }
}

impl Div for BigInt {
    type Output = BigInt;

    fn div(self, other: BigInt) -> BigInt {
        let mut quotient = BigInt::new("0".to_string());
        let mut remainder = BigInt::new("0".to_string());

        for &digit in self.value.iter().rev() {
            remainder.value.insert(0, digit);

            // Remove leading zeros in remainder
            while remainder.value.len() > 1 && remainder.value.last() == Some(&0) {
                remainder.value.pop();
            }

            let mut count = 0;
            while remainder >= other {
                remainder = remainder.subtract(&other);
                count += 1;
            }
            quotient.value.insert(0, count as u8);
        }

        // Remove leading zeros in quotient
        while quotient.value.len() > 1 && quotient.value.last() == Some(&0) {
            quotient.value.pop();
        }

        quotient
    }
}

impl FromStr for BigInt {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BigInt::new(s.to_string()))
    }
}

type ValueType = BigInt;

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
                id: items[0].to_usize(),
                items: vec![],
                operation: Operation::Add,
                added: BigInt::new("0".to_string()),
                divisible: BigInt::new("0".to_string()),
                is_old: false,
                throw_true: 0,
                throw_false: 0,
            });
            idx = items[0].to_usize();
        } else if line.starts_with("Starting") {
            monkeys[idx].items = items;
        } else if line.starts_with("Operation") {
            if line.contains("*") {
                monkeys[idx].operation = Operation::Multi;
            }
            if !items.is_empty() {
                monkeys[idx].added = items[0].clone(); // Clone here
            } else {
                monkeys[idx].is_old = true;
            }
        } else if line.starts_with("Test") {
            monkeys[idx].divisible = items[0].clone(); // Clone here
        } else if line.starts_with("If true") {
            monkeys[idx].throw_true = items[0].to_usize();
        } else if line.starts_with("If false") {
            monkeys[idx].throw_false = items[0].to_usize();
        }
    }

    solve(20, monkeys, &|v| {
        v / BigInt::new("3".to_string())
    });
    // solve(100, monkeys, &|v| {
    //     return v;
    // });
}

fn solve(rounds: usize, mut monkeys: Vec<Monkey>, compute: &dyn Fn(ValueType) -> ValueType) {
    let monkey_count = monkeys.len();
    let mut inspects = vec![0; monkey_count];

    for _ in 0..rounds {
        for i in 0..monkey_count {
            inspects[monkeys[i].id] += monkeys[i].items.len();
            let items: Vec<ValueType> = std::mem::take(&mut monkeys[i].items);
            for item in items {
                let worry_level = compute(new_worry_level(item, &monkeys[i]));
                let throw_index = if worry_level.clone() % monkeys[i].divisible.clone() == BigInt::new("0".to_string()) {
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

fn new_worry_level(worry_level: ValueType, monkey: &Monkey) -> ValueType {
    let added = if monkey.is_old { worry_level.clone() } else { monkey.added.clone() };
    match monkey.operation {
        Operation::Add => worry_level + added,
        Operation::Multi => worry_level * added,
    }
}
