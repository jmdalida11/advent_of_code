struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn create_initial_stack() -> [String; 9] {
    return [
        "FCPGQR".to_string(),
        "WTCP".to_string(),
        "BHPMC".to_string(),
        "LTQSMPR".to_string(),
        "PHJZVGN".to_string(),
        "DPJ".to_string(),
        "LGPZFJTR".to_string(),
        "NLHCFPTJ".to_string(),
        "GVZQHTCW".to_string(),
    ];
}

fn parse_move(m: &str) -> Move {
    let mut moves = m.split(' ').filter_map(|v: &str| {
        if let Ok(v) = v.parse::<usize>() {
            return Some(v);
        }
        return None;
    });
    Move {
        count: moves.next().unwrap(),
        from: moves.next().unwrap() - 1,
        to: moves.next().unwrap() - 1,
    }
}

fn part1(content: String) {
    let mut stacks = create_initial_stack();
    for line in content.lines() {
        let m = parse_move(line);
        for _ in 0..(m.count) {
            stacks[m.to].push(stacks[m.from].chars().last().unwrap());
            stacks[m.from].pop();
        }
    }
    for i in 0..9 {
        print!("{}", stacks[i].chars().last().unwrap());
    }
    println!("");
}

fn part2(content: String) {
    let mut stacks = create_initial_stack();
    for line in content.lines() {
        let m = parse_move(line);
        let v: String = stacks[m.from][stacks[m.from].len() - (m.count)..].to_string();
        stacks[m.to] += &v;
        let mut v: Vec<char> = stacks[m.from].chars().collect();
        v.truncate(v.len() - m.count);
        stacks[m.from] = v.iter().collect();
    }
    for i in 0..9 {
        print!("{}", stacks[i].chars().last().unwrap());
    }
    println!("");
}

fn main() {
    let content = std::fs::read_to_string("input").expect("File 'input' not found!");
    part1(content.clone());
    part2(content.clone());
}

//                         [R] [J] [W]
//             [R] [N]     [T] [T] [C]
// [R]         [P] [G]     [J] [P] [T]
// [Q]     [C] [M] [V]     [F] [F] [H]
// [G] [P] [M] [S] [Z]     [Z] [C] [Q]
// [P] [C] [P] [Q] [J] [J] [P] [H] [Z]
// [C] [T] [H] [T] [H] [P] [G] [L] [V]
// [F] [W] [B] [L] [P] [D] [L] [N] [G]
//  1   2   3   4   5   6   7   8   9
