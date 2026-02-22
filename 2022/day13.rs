use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

#[derive(Debug)]
struct Node {
    value: i32,
    is_list: bool,
    children: Vec<Box<Node>>,
}

impl Node {
    fn new(value: i32, is_list: bool) -> Self {
        Node {
            value,
            children: vec![],
            is_list,
        }
    }
}

#[derive(Debug)]
struct Parser {
    idx: usize,
}

impl Parser {
    fn new() -> Self {
        Parser { idx: 0 }
    }

    fn parse(&mut self, val: String) -> Box<Node> {
        self.idx = 1;
        let vals = val.chars().collect::<Vec<char>>();
        let mut node = Box::new(Node::new(0, true));
        self.traverse(&vals, &mut node);
        return node;
    }

    fn traverse(&mut self, vals: &Vec<char>, node: &mut Box<Node>) {
        while self.idx < vals.len() {
            let c = vals[self.idx];
            self.idx += 1;
            match c {
                '[' => {
                    let mut node_list_child = Box::new(Node::new(0, true));
                    self.traverse(vals, &mut node_list_child);
                    node.children.push(node_list_child);
                }
                ']' => {
                    return;
                }
                digit if c.is_ascii_digit() => {
                    let mut v = digit.to_string();
                    let mut i = self.idx;
                    while vals[i].is_ascii_digit() {
                        v.push(vals[i]);
                        i += 1;
                    }
                    self.idx = i;
                    let value: i32 = v.parse().unwrap();
                    node.children.push(Box::new(Node::new(value, false)));
                }
                _ => {
                    continue;
                }
            }
        }
    }
}

fn is_ordered(left: &Box<Node>, right: &Box<Node>) -> Option<bool> {
    let mut i = 0;
    while i < left.children.len() || i < right.children.len() {
        if i >= left.children.len() {
            return Some(true);
        } else if i >= right.children.len() {
            return Some(false);
        }

        let left_node = &left.children[i];
        let right_node = &right.children[i];
        
        if !left_node.is_list && !right_node.is_list {
            if left_node.value < right_node.value {
                return Some(true);
            } else if left_node.value > right_node.value {
                return Some(false);
            }
        } else if left_node.is_list && right_node.is_list {
            if let Some(ordered) = is_ordered(left_node, right_node) {
                return Some(ordered);
            }
        } else if left_node.is_list ^ right_node.is_list {
            let mut temp_node = Box::new(Node::new(0, true));
            temp_node.children.push(Box::new(Node::new(if !left_node.is_list { left_node.value } else {
                right_node.value
            }, false)));

            if !left_node.is_list {
                let ordered = is_ordered(&temp_node, right_node);
                if ordered.is_some() {
                    return ordered
                }
            } else {
                let ordered = is_ordered(left_node, &temp_node);
                if ordered.is_some() {
                    return ordered
                }
            }
        }

        i += 1;
    }

    return None;
}

fn solve_part1(input: &str) -> i32 {
    let mut parser = Parser::new();
    let mut pairs: Vec<(String, String)> = vec![];
    let mut pair = (String::new(), String::new());

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        if pair.0.is_empty() {
            pair.0 = line.trim().to_string();
        } else {
            pair.1 = line.trim().to_string();
            pairs.push(pair);
            pair = (String::new(), String::new());
        }
    }
    
    let mut ans: i32 = 0;

    for i in 0..pairs.len() {
        let left_node = parser.parse(pairs[i].0.clone());
        let right_node = parser.parse(pairs[i].1.clone());
        if let Some(ordered) = is_ordered(&left_node, &right_node) {
            if ordered {
                // println!("Hey {}" , i + 1);
                ans += i as i32 + 1;
            }
        }
    }
    
    ans
}

fn solve_part2(_input: &str) -> i32 {
    // Implement the solution for part 2
    0
}